use model::messages::hello;
use repository::dummy::DummyRepository;
use tracing::warn;

#[cfg_attr(any(test, feature = "faux"), faux::create)]
#[derive(Clone)]
pub struct Service {
    repo: DummyRepository,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[cfg_attr(any(test, feature = "faux"), faux::methods)]
impl Service {
    #[must_use]
    pub const fn new(repo: DummyRepository) -> Self {
        Self { repo }
    }

    #[must_use]
    pub fn message(&self) -> String {
        "Hello, World from Service!".to_string()
    }

    /// access the repository.
    ///
    /// # Errors
    ///
    /// Passes through DB errors
    pub async fn number(&self) -> Result<i64, repository::SqlxError> {
        self.repo.fetch(4).await
    }

    /// example for a message handler.
    ///
    /// Might also go into another service, just an example.
    ///
    /// Will fail with its own error-type
    pub async fn handle(&self, m: hello::Message) -> Result<(), Error> {
        match m {
            hello::Message::Name(name) => {
                self.repo.insert(&name).await.unwrap();
            }
            hello::Message::Tombstone => warn!("someone is dead"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Service;
    use repository::dummy::DummyRepository;

    #[test]
    fn test_message() {
        let repo = DummyRepository::faux();

        let service = Service::new(repo);
        let actual = service.message();
        let expected = "Hello, World from Service!".to_string();

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_number() {
        let mut repo = DummyRepository::faux();
        faux::when!(repo.fetch).then(|_| Ok(42));

        let service = Service::new(repo);
        let actual = service.number().await.unwrap();
        let expected = 42;

        assert_eq!(actual, expected);
    }
}
