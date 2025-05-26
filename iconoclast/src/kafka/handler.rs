use std::error::Error;

/// A kafka message handler.
///
/// Gets passed decoded Kafka Messages of type `Self::Message`
///
/// Returns an application error `AE` if processing fails
pub trait MessageHandler<AE: Error + Send + Sync> {
    /// The type of messages this handler handles.
    ///
    /// This is typically an enum, where each variant is a message for a topic.
    type Message;

    fn handle(&self, kafka_message: Self::Message) -> impl Future<Output = Result<(), AE>>;
}
