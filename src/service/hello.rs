#[double]
use crate::dal::dummy::DummyRepository;
use mockall_double::double;

pub struct Service {
    repo: DummyRepository,
}

#[cfg_attr(test, mockall::automock)]
impl Service {
    pub fn new(repo: DummyRepository) -> Self {
        Self { repo }
    }

    pub fn message(&self) -> String {
        "Hello, World from Service!".to_string()
    }

    pub async fn number(&self) -> Result<i64, sqlx::Error> {
        self.repo.fetch(4).await
    }
}

#[cfg(test)]
mod test {
    use crate::dal::dummy::MockDummyRepository;
    use crate::service::hello;

    #[test]
    fn test_message() {
        let repo = MockDummyRepository::default();

        let service = hello::Service::new(repo);
        let actual = service.message();
        let expected = "Hello, World from Service!".to_string();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_number() {
        let mut repo = MockDummyRepository::default();
        repo.expect_fetch().returning(|_| Ok(42));

        let service = hello::Service::new(repo);
        let actual = service.number().await.unwrap();
        let expected = 42;

        assert_eq!(actual, expected);
    }
}
