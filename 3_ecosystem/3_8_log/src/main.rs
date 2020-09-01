use std::fs::OpenOptions;

use chrono::{SecondsFormat, Utc};
use slog::{o, Drain, FnValue, Level::Warning, Logger, PushFnValue};
use slog_json::JsonBuilder;
use slog_scope::{error, info, warn};

fn add_kv<T: std::io::Write>(builder: JsonBuilder<T>) -> JsonBuilder<T> {
    builder.add_key_value(o!(
        "time" => PushFnValue(move |_, ser| {
            ser.emit(Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true))
        }),
        "lvl" => FnValue(move |r| r.level().as_str()),
        "msg" => PushFnValue(move |r, ser| ser.emit(r.msg())),
    ))
}

fn init_logger<T, U>(io_warn: T, io_info: U) -> Logger
where
    T: std::io::Write + Send + 'static,
    U: std::io::Write + Send + 'static,
{
    let warn = add_kv(slog_json::Json::new(io_warn)).build();
    let warn = slog::Filter(warn, |r| r.level().is_at_least(Warning)).fuse();

    let info = add_kv(slog_json::Json::new(io_info)).build();
    let info = slog::Filter(info, |r| r.level() > Warning).fuse();

    let drain = slog::Duplicate::new(warn, info).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}

fn main() {
    let root = init_logger(std::io::stderr(), std::io::stdout());
    let _guard = slog_scope::set_global_logger(root);

    info!("global info");
    warn!("global warn");
    error!("global err");

    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("access.log")
        .unwrap();

    let scoped =
        init_logger(log_file.try_clone().unwrap(), log_file).new(o!("file" => "access.log"));

    slog_scope::scope(&scoped, || {
        info!("local info");
        warn!("local warn");
        error!("local err");
    });
}
