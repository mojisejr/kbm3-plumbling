use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::io::Write;

pub fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    Builder::from_env(env)
        .format(|buf, record| {
            let level = record.level();
            let level_color = match level {
                log::Level::Error => "\x1b[31m", // Red
                log::Level::Warn => "\x1b[33m",  // Yellow
                log::Level::Info => "\x1b[32m",  // Green
                log::Level::Debug => "\x1b[36m", // Cyan
                log::Level::Trace => "\x1b[35m", // Magenta
            };
            let reset = "\x1b[0m";

            writeln!(
                buf,
                "{}[{}{}{}] {} - {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_color,
                level,
                reset,
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized");
}

// Convenience functions for logging
pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_warn(message: &str) {
    warn!("{}", message);
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

pub fn log_trace(message: &str) {
    trace!("{}", message);
}
