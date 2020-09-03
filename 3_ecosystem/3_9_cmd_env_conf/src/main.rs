use std::borrow::Cow;
use std::time::Duration;

use clap::{load_yaml, App};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    mode: Mode,
    server: Server,
    db: MySQL,
    log: LogApp,
    background: Background,
}

impl Settings {
    fn new<'a>(file: impl Into<&'a str>) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let filename = "3_ecosystem/3_9_cmd_env_conf/".to_string() + file.into();

        s.set_default("mode.debug", false)?
            .set_default("server.external_url", "http://127.0.0.1")?
            .set_default("server.http_port", 8081)?
            .set_default("server.grpc_port", 8082)?
            .set_default("server.healthz_port", 10025)?
            .set_default("server.metrics_port", 9199)?
            .set_default("db.mysql.host", "127.0.0.1")?
            .set_default("db.mysql.port", 3306)?
            .set_default("db.mysql.dating", "default")?
            .set_default("db.mysql.user", "root")?
            .set_default("db.mysql.pass", "")?
            .set_default("db.mysql.connections.max_idle", 30)?
            .set_default("db.mysql.connections.max_open", 30)?
            .set_default("log.app.level", "info")?
            .set_default("background.watchdog.period", "5s")?
            .set_default("background.watchdog.limit", 10)?
            .set_default("background.watchdog.lock_timeout", "4s")?;

        s.set("mode.debug", true)?;

        s.merge(File::with_name(&filename))?;
        s.merge(Environment::with_prefix("conf"))?;

        s.try_into()
    }
}

#[derive(Debug, Deserialize)]
struct MySQL {
    mysql: Database,
}

#[derive(Debug, Deserialize)]
struct LogApp {
    app: Log,
}

#[derive(Debug, Deserialize)]
struct Background {
    watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
struct Mode {
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct Server {
    external_url: Cow<'static, str>,
    http_port: u16,
    grpc_port: u16,
    healthz_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Deserialize)]
struct Database {
    host: Cow<'static, str>,
    port: u16,
    dating: Cow<'static, str>,
    user: Cow<'static, str>,
    pass: Cow<'static, str>,
    connections: Connections,
}

#[derive(Debug, Deserialize)]
struct Connections {
    max_idle: u16,
    max_open: u16,
}

#[derive(Debug, Deserialize)]
struct Log {
    level: LogLevel,
}

#[derive(Debug, Deserialize)]
struct Watchdog {
    #[serde(with = "humantime_serde")]
    period: Duration,
    limit: u16,
    #[serde(with = "humantime_serde")]
    lock_timeout: Duration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let conf_file = matches.value_of("config").unwrap();

    let settings = Settings::new(conf_file).unwrap();

    if matches.is_present("debug") {
        dbg!(settings);
    }
}
