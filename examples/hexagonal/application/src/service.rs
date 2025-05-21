use crate::outbound::TaskRepository;

pub struct TodoService<T: TaskRepository> {
    repository: T,
}

impl<T: TaskRepository> TodoService<T> {
    pub const fn new(repository: T) -> Self {
        Self { repository }
    }
}
