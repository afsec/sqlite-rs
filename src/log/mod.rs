use crate::result::SqliteError;
use std::{fmt::Display, str::FromStr, sync::OnceLock};

pub(crate) static LOGGER: OnceLock<LogLevel> = OnceLock::new();

#[derive(Debug, Default)]
pub(crate) struct EnvLogger;

impl EnvLogger {
  pub(crate) fn init() {
    use std::env;
    let maybe_loglevel_str = env::var("RUST_LOG").unwrap_or_default();
    LOGGER.get_or_init(|| maybe_loglevel_str.parse().unwrap_or_default());
  }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LogLevel {
  #[default]
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

impl FromStr for LogLevel {
  type Err = SqliteError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let log_level = match s {
      "WARN" | "warn" => LogLevel::Warn,
      "ERROR" | "error" => LogLevel::Error,
      "INFO" | "info" => LogLevel::Info,
      "DEBUG" | "debug" => LogLevel::Debug,
      "TRACE" | "trace" => LogLevel::Trace,
      _ => {
        return Err(SqliteError::Custom(
          "Error on parsing RUST_LOG value".into(),
        ))
      }
    };
    Ok(log_level)
  }
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      LogLevel::Error => "ERROR",
      LogLevel::Warn => "WARN",
      LogLevel::Info => "INFO",
      LogLevel::Debug => "DEBUG",
      LogLevel::Trace => "TRACE",
    };
    write!(f, "{s}")
  }
}
