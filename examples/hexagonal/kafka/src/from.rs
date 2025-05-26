use crate::topic;
use crate::topic::todo;
use domain::{TaskId, event};
use uuid::Uuid;

impl From<todo::KafkaMessage> for event::Task {
    fn from(m: todo::KafkaMessage) -> Self {
        match m {
            todo::KafkaMessage::Task(topic::KafkaTopicMessage { key, payload, .. }) => {
                let key = key.unwrap_or_else(Uuid::new_v4);
                event::Task::Added(TaskId(key), payload)
            }
            todo::KafkaMessage::Tombstone(topic::KafkaTopicMessage { key, .. }) => {
                event::Task::Deleted(TaskId(key))
            }
        }
    }
}
