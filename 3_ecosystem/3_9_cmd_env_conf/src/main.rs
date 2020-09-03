use std::borrow::Cow;
use std::time::Duration;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use structopt::StructOpt;

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
    #[serde(default = "Server::default_external_url")]
    external_url: Cow<'static, str>,
    #[serde(default = "Server::default_http_port")]
    http_port: u16,
    #[serde(default = "Server::default_grpc_port")]
    grpc_port: u16,
    #[serde(default = "Server::default_healthz_port")]
    healthz_port: u16,
    #[serde(default = "Server::default_metrics_port")]
    metrics_port: u16,
}

impl Server {
    fn default_external_url() -> Cow<'static, str> {
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
}

#[derive(Debug, Deserialize)]
struct Database {
    #[serde(default = "Database::default_host")]
    host: Cow<'static, str>,
    #[serde(default = "Database::default_port")]
    port: u16,
    #[serde(default = "Database::default_dating")]
    dating: Cow<'static, str>,
    #[serde(default = "Database::default_user")]
    user: Cow<'static, str>,
    #[serde(default = "Database::default_pass")]
    pass: Cow<'static, str>,
    connections: Connections,
}

impl Database {
    fn default_host() -> Cow<'static, str> {
        "127.0.0.1".into()
    }

    fn default_port() -> u16 {
        3306
    }

    fn default_dating() -> Cow<'static, str> {
        "default".into()
    }

    fn default_user() -> Cow<'static, str> {
        "root".into()
    }

    fn default_pass() -> Cow<'static, str> {
        "".into()
    }
}

#[derive(Debug, Deserialize)]
struct Connections {
    #[serde(default = "Connections::default_max_idle")]
    max_idle: u16,
    #[serde(default = "Connections::default_max_open")]
    max_open: u16,
}

impl Connections {
    fn default_max_idle() -> u16 {
        30
    }

    fn default_max_open() -> u16 {
        30
    }
}

#[derive(Debug, Deserialize)]
struct Log {
    #[serde(default = "Log::default_level")]
    level: LogLevel,
}

impl Log {
    fn default_level() -> LogLevel {
        LogLevel::Info
    }
}

#[derive(Debug, Deserialize)]
struct Watchdog {
    #[serde(with = "humantime_serde", default = "Watchdog::default_period")]
    period: Duration,
    #[serde(default = "Watchdog::default_limit")]
    limit: u16,
    #[serde(with = "humantime_serde", default = "Watchdog::default_lock_timeout")]
    lock_timeout: Duration,
}

impl Watchdog {
    fn default_period() -> Duration {
        Duration::from_secs(5)
    }

    fn default_limit() -> u16 {
        10
    }

    fn default_lock_timeout() -> Duration {
        Duration::from_secs(4)
    }
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "step_3_9",
    about = "Prints its configuration to STDOUT.",
    version = "0.1.0"
)]
struct Options {
    #[structopt(short, long, help = "Enables debug mode")]
    debug: bool,

    #[structopt(
        short,
        long = "conf",
        value_name = "conf",
        env = "CONF_FILE",
        default_value = "config.toml",
        takes_value = true,
        help = "Path to configuration file"
    )]
    config: String,
}

fn main() {
    let options: Options = Options::from_args();

    let settings = Settings::new(options.config.as_str()).unwrap();

    if options.debug {
        dbg!(settings);
    }
}
