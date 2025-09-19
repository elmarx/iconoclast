use crate::kafka::{self, MessageHandler, StreamError};

use futures::TryStreamExt;
use rdkafka::consumer::{
    CommitMode, Consumer as ConsumerExt, ConsumerContext, DefaultConsumerContext, StreamConsumer,
};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, ClientContext};
use std::error::Error;
use std::marker::PhantomData;
use tokio_stream::StreamExt;
use tracing::warn;

pub struct Consumer<M, KM, DE, AE, CC = DefaultConsumerContext>
where
    M: MessageHandler<AE, Message = KM> + Send + Sync,
    AE: Error + Send + Sync,
    DE: Error + Send + Sync,
    KM: for<'a> TryFrom<&'a BorrowedMessage<'a>, Error = DE> + Send + Sync,
    CC: ClientContext + ConsumerContext + Send + Sync,
{
    /// actual rdkafka consumer
    consumer: StreamConsumer<CC>,
    application_error: PhantomData<AE>,
    /// message handler that receives successfully decoded kafka messages
    handler: M,
}

impl<M, KM, DE, AE> Consumer<M, KM, DE, AE, DefaultConsumerContext>
where
    M: MessageHandler<AE, Message = KM> + Send + Sync,
    AE: Error + Send + Sync,
    DE: Error + Send + Sync,
    KM: for<'a> TryFrom<&'a BorrowedMessage<'a>, Error = DE> + Send + Sync,
{
    /// Create a new Kafka consumer with the given configuration and message handler
    /// using the default context.
    pub fn new(config: &kafka::Config, handler: M) -> Result<Self, KafkaError> {
        Self::new_with_context(config, handler, DefaultConsumerContext)
    }
}

// This file is copied from the `iconoclast` crate, with some modifications to use a custom `ClientContext`.
impl<M, KM, DE, AE, CC> Consumer<M, KM, DE, AE, CC>
where
    M: MessageHandler<AE, Message = KM> + Send + Sync,
    AE: Error + Send + Sync,
    DE: Error + Send + Sync,
    KM: for<'a> TryFrom<&'a BorrowedMessage<'a>, Error = DE> + Send + Sync,
    CC: ClientContext + ConsumerContext + Send + Sync + 'static,
{
    /// Create a new Kafka consumer with the given configuration, message handler, and custom context.
    pub fn new_with_context(
        config: &kafka::Config,
        handler: M,
        context: CC,
    ) -> Result<Self, KafkaError> {
        let config = config.clone();
        let mut cfg = ClientConfig::new();
        cfg.extend(config.properties.into_iter().map(|(k, v)| (k, v.into())));
        cfg.extend(config.env_properties);

        Ok(Self {
            consumer: cfg.create_with_context(context)?,
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
