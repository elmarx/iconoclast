use crate::kafka;
use crate::kafka::{MessageHandler, StreamError};
use futures::TryStreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer as ConsumerExt, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use std::error::Error;
use std::marker::PhantomData;
use tokio_stream::StreamExt;
use tracing::warn;

pub struct Consumer<M, KM, DE, AE>
where
    M: MessageHandler<AE, Message = KM> + Send + Sync,
    AE: Error + Send + Sync,
    DE: Error + Send + Sync,
    KM: for<'a> TryFrom<&'a BorrowedMessage<'a>, Error = DE>,
{
    /// actual rdkafka consumer
    consumer: StreamConsumer,
    application_error: PhantomData<AE>,
    /// message handler that receives successfully decoded kafka messages
    handler: M,
}

impl<M, KM, DE, AE> Consumer<M, KM, DE, AE>
where
    M: MessageHandler<AE, Message = KM> + Send + Sync,
    AE: Error + Send + Sync,
    DE: Error + Send + Sync,
    KM: for<'a> TryFrom<&'a BorrowedMessage<'a>, Error = DE> + Send + Sync,
{
    pub fn new(config: &kafka::Config, handler: M) -> Result<Self, KafkaError> {
        let config = config.clone();
        let mut cfg = ClientConfig::new();
        cfg.extend(config.properties.into_iter().map(|(k, v)| (k, v.into())));
        cfg.extend(config.env_properties);

        let rdkafka_consumer: StreamConsumer = cfg.create()?;

        Ok(Self {
            consumer: rdkafka_consumer,
            application_error: PhantomData,
            handler,
        })
    }

    /// start consuming by subscribing to topics and polling for items (via [`rdkafka::consumer::StreamConsumer`])
    pub async fn start(&self) -> Result<(), StreamError<DE, AE>> {
        let topics = M::topics();

        if topics.is_empty() {
            warn!("NOT starting kafka-consumer as list of topics is empty.");
            return Ok(());
        }

        self.consumer.subscribe(topics)?;

        self.consumer
            .stream()
            .then(async |m| -> Result<BorrowedMessage, StreamError<DE, AE>> {
                let bm = m?;
                let message = KM::try_from(&bm).map_err(StreamError::Decode)?;

                self.handler
                    .handle(message)
                    .await
                    .map_err(StreamError::Application)?;

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
