use crate::TaskId;

#[derive(Debug)]
pub enum Task {
    Deleted(TaskId),
    Added(TaskId, String),
}
