use crate::consumer::topic::{ParseError, Payload};
use crate::init::settings;
use crate::service::hello::Error as HelloError;
#[mockall_double::double]
use crate::service::hello::Service as HelloService;
use futures::TryStreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use thiserror::Error;
use tokio_stream::StreamExt;

pub mod topic;

pub mod hello;

/// the payload used for kafka messages
pub type KafkaPayload<'a> = Option<&'a [u8]>;

/// all errors that may happen during strem processing
#[derive(Error, Debug)]
pub enum StreamError {
    #[error(transparent)]
    Kafka(#[from] KafkaError),
    #[error(transparent)]
    HelloService(#[from] HelloError),
    #[error(transparent)]
    Parse(#[from] ParseError),
}

pub struct IconoclastConsumer {
    consumer: StreamConsumer,
    service: HelloService,
}

impl IconoclastConsumer {
    pub fn new(config: &settings::Kafka, service: HelloService) -> Result<Self, KafkaError> {
        let config = config.clone();
        let mut cfg = ClientConfig::new();
        cfg.extend(config.properties.into_iter().map(|(k, v)| (k, v.into())));
        cfg.extend(config.env_properties);

        let consumer: StreamConsumer = cfg.create()?;

        Ok(Self { consumer, service })
    }

    pub async fn run(&self) -> Result<(), StreamError> {
        self.consumer.subscribe(topic::TOPICS)?;

        // TODO: start multiple stream-consumers to run in parallel (see https://github.com/fede1024/rust-rdkafka/blob/0b35424129f394e746c2e40519169595d8ac240c/examples/asynchronous_processing.rs#L179)
        self.consumer
            .stream()
            .then(async |m| -> Result<BorrowedMessage, StreamError> {
                let bm = m?;
                let payload: Payload = (&bm).try_into()?;

                match payload {
                    Payload::Hello(p) => self.service.handle(p).await?,
                };

                Ok(bm)
            })
            .try_for_each(async |bm| {
                self.consumer
                    .commit_message(&bm, CommitMode::Async)
                    .map_err(StreamError::Kafka)
            })
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::consumer::IconoclastConsumer;
    use crate::consumer::hello::HelloMessage;
    use crate::init::settings;
    use crate::service::hello::MockService;
    use rdkafka::ClientConfig;
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use std::collections::HashMap;
    use tokio::sync::oneshot;

    async fn publish(brokers: String, topic: &str, key: &str, payload: &str) {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .unwrap();

        let record = FutureRecord::to(topic).payload(payload).key(key);
        producer.send(record, None).await.unwrap();
    }

    /// test that a message sent through kafka ends up in call to the (mock-) Service
    #[tokio::test]
    async fn smoketest() {
        // set up a mock that sends the value (once), as we need to wait for the value
        let (tx, rx) = oneshot::channel::<HelloMessage>();
        let mut service = MockService::default();
        service.expect_handle().return_once(|m| {
            tx.send(m).unwrap();
            Ok(())
        });

        // the test never exits. mem::drop() on the cluster doesn't help, this… seems to help somehow :/
        let cluster = Box::leak(Box::new(rdkafka::mocking::MockCluster::new(3).unwrap()));
        cluster.create_topic(super::hello::TOPIC, 12, 3).unwrap();

        let config = settings::Kafka {
            env_properties: vec![
                ("bootstrap.servers".to_string(), cluster.bootstrap_servers()),
                ("group.id".to_string(), "smoketest".to_string()),
                ("auto.offset.reset".to_string(), "earliest".to_string()),
                ("enable.auto.commit".to_string(), "false".to_string()),
            ],
            properties: HashMap::new(),
        };

        let consumer = IconoclastConsumer::new(&config, service).unwrap();

        let task = tokio::task::spawn(async move { consumer.run().await });

        publish(
            cluster.bootstrap_servers(),
            super::hello::TOPIC,
            "1",
            "Ferris",
        )
        .await;

        let actual = rx.await.unwrap();
        assert_eq!(HelloMessage::Name("Ferris".to_string()), actual);
        task.abort();
    }
}
