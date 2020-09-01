use clap::{load_yaml, App};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct Mode {
    #[serde(default)]
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct Server<'a> {
    #[serde(default = "default_url")]
    external_url: Cow<'a, str>,
    #[serde(default = "default_http_port")]
    http_port: u16,
    #[serde(default = "default_grpc_port")]
    grpc_port: u16,
    #[serde(default = "default_healthz_port")]
    healthz_port: u16,
    #[serde(default = "default_metrics_port")]
    metrics_port: u16,
}

fn default_url() -> Cow<str> {
    "http://127.0.0.1".into()
}

fn default_http_port() -> u16 {
    8081
}

fn default_grpc_port() -> u16 {
    8082
}

fn default_healthz_port() -> u16 {
    10025
}

fn default_metrics_port() -> u16 {
    9199
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let conf_file = matches.value_of("config").unwrap();
    dbg!(conf_file);
}
