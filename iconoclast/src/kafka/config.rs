//! helpers to configure kafka conveniently via TOML and environment variables

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(skip)]
    pub env_properties: Vec<(String, String)>,
    #[serde(flatten, default)]
    pub properties: HashMap<String, PropertyValue>,
}

/// collect env-vars into kafka-properties
/// e.g. turns `KAFKA_BOOTSTRAP_SERVERS` into `bootstrap.servers`
pub fn from_env(env_vars: impl Iterator<Item = (String, String)>) -> Vec<(String, String)> {
    env_vars
        .filter_map(|(k, v)| {
            k.strip_prefix("KAFKA_")
                .map(|prop| (prop.replace('_', ".").to_lowercase(), v.to_string()))
        })
        .collect()
}

/// type to accept all values allowed by rdkafka.
/// [`rdkafka::config::ClientConfig`] expects all properties as `Into<String>`, this enables to write numbers (and booleans) into toml without quotes
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Bool(bool),
    Integer(i64),
}

impl From<&PropertyValue> for String {
    fn from(v: &PropertyValue) -> Self {
        match v {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Bool(b) => b.to_string(),
            PropertyValue::Integer(i) => i.to_string(),
        }
    }
}

impl From<PropertyValue> for String {
    fn from(v: PropertyValue) -> Self {
        match v {
            PropertyValue::String(s) => s,
            PropertyValue::Bool(b) => b.to_string(),
            PropertyValue::Integer(i) => i.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::from_env;

    #[test]
    fn test_kafka_from_env() {
        let env_vars = vec![
            ("XYZ".to_string(), "short".to_string()),
            (
                "KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "localhost:9092".to_string(),
            ),
            ("KAFKA_GROUP_ID".to_string(), "iconoclast".to_string()),
            (
                "KAFKA_SSL_CA_LOCATION".to_string(),
                "/var/run/secrets/ca.pem".to_string(),
            ),
        ];

        let actual = from_env(env_vars.into_iter());
        let expected: Vec<_> = vec![
            ("bootstrap.servers", "localhost:9092"),
            ("group.id", "iconoclast"),
            ("ssl.ca.location", "/var/run/secrets/ca.pem"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();

        assert_eq!(actual, expected);
    }
}
