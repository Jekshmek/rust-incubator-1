use core::fmt;
use std::fs;

use config::{ConfigError, Environment};
use once_cell::sync::Lazy;
use serde::{de, Deserialize, Deserializer};
use serde_with::with_prefix;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().unwrap());

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(flatten, with = "database_")]
    pub database: Database,
    #[serde(flatten, with = "server_")]
    pub server: Server,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
    #[serde(deserialize_with = "de_u32")]
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub url: String,
    pub secret_key: String,
    pub domain: String,
}

with_prefix!(server_ "server_");
with_prefix!(database_ "database_");

fn de_u32<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    de.deserialize_str(U32Visitor)
}

struct U32Visitor;

impl<'de> de::Visitor<'de> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("usize string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse::<u32>().map_err(|_| E::custom("String isn`t u32"))
    }
}

impl Config {
    fn new() -> Result<Self, ConfigError> {
        dotenv_setup().ok();

        let mut c = config::Config::new();
        c.merge(Environment::new())?;
        c.try_into()
    }
}

fn dotenv_setup() -> dotenv::Result<()> {
    let mut env_path = fs::canonicalize("./").unwrap();
    env_path.push("3_ecosystem");
    env_path.push(".env");
    dotenv::from_path(env_path.as_path())
}
