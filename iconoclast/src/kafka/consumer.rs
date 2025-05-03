use crate::kafka;
use crate::kafka::{MessageHandler, StreamError};
use futures::TryStreamExt;
use rdkafka::consumer::{CommitMode, Consumer as ConsumerExt, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, Message};
use std::error::Error;
use std::marker::PhantomData;
use tokio_stream::StreamExt;

pub struct Consumer<M, P, PE, LE>
where
    M: MessageHandler<P, LE> + Send + Sync,
    LE: Error + Send + Sync,
    PE: Error + Send + Sync,
    P: for<'a> TryFrom<(&'a str, Option<&'a [u8]>), Error = PE>,
{
    consumer: StreamConsumer,
    payload: PhantomData<P>,
    parse_errors: PhantomData<PE>,
    logic_errors: PhantomData<LE>,
    handler: M,
    topics: &'static [&'static str],
}

impl<M, P, PE, LE> Consumer<M, P, PE, LE>
where
    M: MessageHandler<P, LE> + Send + Sync,
    LE: Error + Send + Sync,
    PE: Error + Send + Sync,
    P: for<'a> TryFrom<(&'a str, Option<&'a [u8]>), Error = PE> + Send + Sync,
{
    pub fn new(
        config: &kafka::Config,
        topics: &'static [&'static str],
        handler: M,
    ) -> Result<Self, KafkaError> {
        let config = config.clone();
        let mut cfg = ClientConfig::new();
        cfg.extend(config.properties.into_iter().map(|(k, v)| (k, v.into())));
        cfg.extend(config.env_properties);

        let consumer: StreamConsumer = cfg.create()?;

        Ok(Self {
            consumer,
            payload: PhantomData,
            parse_errors: PhantomData,
            logic_errors: PhantomData,
            topics,
            handler,
        })
    }

    pub async fn start(&self) -> Result<(), StreamError<PE, LE>> {
        self.consumer.subscribe(self.topics)?;

        self.consumer
            .stream()
            .then(async |m| -> Result<BorrowedMessage, StreamError<PE, LE>> {
                let bm = m?;
                let payload =
                    P::try_from((bm.topic(), bm.payload())).map_err(StreamError::Parse)?;

                self.handler
                    .handle(payload)
                    .await
                    .map_err(StreamError::Logic)?;

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
