use model::messages::hello;
use repository::dummy::DummyRepository;
use tracing::{info, warn};

#[derive(Clone)]
pub struct Service {
    repo: DummyRepository,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[cfg_attr(test, mockall::automock)]
impl Service {
    pub fn new(repo: DummyRepository) -> Self {
        Self { repo }
    }

    pub fn message(&self) -> String {
        "Hello, World from Service!".to_string()
    }

    pub async fn number(&self) -> Result<i64, repository::SqlxError> {
        self.repo.fetch(4).await
    }

    pub async fn handle(&self, m: hello::Message) -> Result<(), Error> {
        match m {
            hello::Message::Name(name) => info!("Hello {name}!"),
            hello::Message::Tombstone => warn!("someone is dead"),
        }

        Ok(())
    }
}

#[cfg(test)]
impl Clone for MockService {
    fn clone(&self) -> Self {
        // cloning a mock doesn't make sense
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::service::hello;
    use repository::dummy::DummyRepository;

    #[test]
    fn test_message() {
        let repo = DummyRepository::faux();

        let service = hello::Service::new(repo);
        let actual = service.message();
        let expected = "Hello, World from Service!".to_string();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_number() {
        let mut repo = DummyRepository::faux();
        faux::when!(repo.fetch).then(|_| Ok(42));

        let service = hello::Service::new(repo);
        let actual = service.number().await.unwrap();
        let expected = 42;

        assert_eq!(actual, expected);
    }
}
