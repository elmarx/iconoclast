use crate::kafka::{MessageHandler, consumer};
use consumer::Consumer;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::collections::HashMap;
use std::convert::Infallible;
use tokio::sync::oneshot;

async fn publish(brokers: String, topic: &str, key: &str, payload: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .unwrap();

    let record = FutureRecord::to(topic).payload(payload).key(key);
    producer.send(record, None).await.unwrap();
}

#[faux::create]
struct TestHandler {}

#[faux::methods]
impl MessageHandler<Pl, Infallible> for TestHandler {
    async fn handle(&self, _payload: Pl) -> Result<(), Infallible> {
        unimplemented!("mock")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pl(String);

impl TryFrom<(&str, Option<&[u8]>)> for Pl {
    type Error = Infallible;

    fn try_from((_topic, payload): (&str, Option<&[u8]>)) -> Result<Self, Self::Error> {
        Ok(Self(
            String::from_utf8(Vec::from(payload.unwrap())).unwrap(),
        ))
    }
}

/// test that a message sent through kafka ends up in call to the (mock-) Service
#[tokio::test]
async fn smoketest() {
    // set up a mock that sends the value (once), as we need to wait for the value
    let (tx, rx) = oneshot::channel::<Pl>();
    // let tx = RefCell::new(Some(tx));
    let mut service = TestHandler::faux();
    faux::when!(service.handle).once().then(move |m| {
        tx.send(m).unwrap();
        Ok(())
    });

    // the test never exits. mem::drop() on the cluster doesn't help, this… seems to help somehow :/
    let cluster = Box::leak(Box::new(rdkafka::mocking::MockCluster::new(3).unwrap()));
    cluster.create_topic("hello", 12, 3).unwrap();

    let config = super::Config {
        env_properties: vec![
            ("bootstrap.servers".to_string(), cluster.bootstrap_servers()),
            ("group.id".to_string(), "smoketest".to_string()),
            ("auto.offset.reset".to_string(), "earliest".to_string()),
            ("enable.auto.commit".to_string(), "false".to_string()),
        ],
        properties: HashMap::new(),
    };

    let consumer = Consumer::new(&config, &["hello"], service).unwrap();

    let task = tokio::task::spawn(async move { consumer.start().await });

    publish(cluster.bootstrap_servers(), "hello", "1", "Ferris").await;

    let actual = rx.await.unwrap();
    assert_eq!(Pl("Ferris".to_string()), actual);
    task.abort();
}
