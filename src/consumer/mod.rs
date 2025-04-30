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
                }

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
