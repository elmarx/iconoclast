use uuid::Uuid;

pub mod event;
pub mod from;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TaskId(pub Uuid);

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
}

#[cfg(test)]
mod test {
    use crate::TaskId;

    #[test]
    fn test_task_id_new() {
        let sample = TaskId::new();

        let actual = sample.0.to_string();

        assert_ne!(actual, "00000000-0000-0000-0000-000000000000".to_string());
    }

    #[test]
    fn test_task_id_default() {
        let sample = TaskId::default();

        let actual = sample.0.to_string();

        assert_ne!(actual, "00000000-0000-0000-0000-000000000000".to_string());
    }
}
