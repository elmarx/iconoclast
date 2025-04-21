#[derive(Default)]
pub struct Service {}

#[cfg_attr(test, mockall::automock)]
impl Service {
    pub fn message(&self) -> String {
        "Hello, World from Service!".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::service::hello;

    #[test]
    fn test_message() {
        let service = hello::Service::default();
        let actual = service.message();
        let expected = "Hello, World from Service!".to_string();

        assert_eq!(actual, expected);
    }
}
