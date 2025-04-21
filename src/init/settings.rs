use std::convert::Infallible;

#[derive(Debug)]
pub struct Settings {}

impl Settings {
    pub fn emerge() -> Result<Self, Infallible> {
        Ok(Settings {})
    }
}
