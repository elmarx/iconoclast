use std::convert::Infallible;

#[derive(Debug)]
pub struct Settings {
    pub db_url: String,
}

impl Settings {
    pub fn emerge() -> Result<Self, Infallible> {
        Ok(Settings {
            db_url: "postgres://iconoclast:iconoclast@localhost/iconoclast".to_string(),
        })
    }
}
