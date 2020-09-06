use std::fs;

use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv_setup().ok();

        let mut c = config::Config::new();

        c.merge(Environment::new())?;

        c.try_into()
    }
}

fn dotenv_setup() -> dotenv::Result<()> {
    let mut env_path = fs::canonicalize("./").unwrap();
    env_path.push("3_ecosystem");
    dotenv::from_path(env_path.as_path())
}
