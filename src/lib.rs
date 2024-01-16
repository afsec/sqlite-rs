#![forbid(unsafe_code, non_ascii_idents)]

//! # SQLite arquitecture
//! *Reference:* https://www.sqlite.org/arch.html

use crate::io::SqliteIo;
use crate::pager::SqlitePager;
use crate::result::SqliteResult;
use crate::runtime::SqliteRuntime;
use std::sync::OnceLock;

pub mod header;
pub mod io;
#[cfg(feature = "log")]
pub(crate) mod log;
#[macro_use]
pub(crate) mod log_macros;
pub mod pager;
pub mod result;
pub mod runtime;
pub mod traits;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct SqliteConnection {
  runtime: SqliteRuntime,
}
static VERSION_NUMBER: OnceLock<u32> = OnceLock::new();

impl SqliteConnection {
  pub fn open(conn_str: impl AsRef<str>) -> SqliteResult<Self> {
    crate::log::EnvLogger::init();

    VERSION_NUMBER.get_or_init(|| {
      let mut s = env!("CARGO_PKG_VERSION").split('.');
      let release = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
      let major = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);
      let minor = s.next().and_then(|x| x.parse().ok()).unwrap_or(0u32);

      (10_000 * release) + (100 * major) + minor
    });

    trace!("Openning SQliteIo [{}]...", conn_str.as_ref());
    let io = SqliteIo::open(conn_str)?;
    trace!("SQliteIo started: [{io:?}].");
    trace!("Connecting SqlitePager...");
    let pager = SqlitePager::connect(io)?;
    trace!("SQliteIo started: [{pager:?}].");
    trace!("Starting SqliteRuntime...");
    let runtime = SqliteRuntime::start(pager)?;
    trace!("SqliteRuntime started: [{runtime:?}].");

    Ok(Self { runtime })
  }
  #[cfg(all(debug_assertions, test))]
  pub fn open_sample() -> SqliteResult<Self> {
    let conn_str = ":memory:";
    crate::log::EnvLogger::init();
    trace!("Openning SQliteIo [{}]...", conn_str);
    // TODO: Remove Default
    let io = SqliteIo::default();
    trace!("SQliteIo started: [{io:?}].");
    trace!("Connecting SqlitePager...");
    let pager = SqlitePager::connect(io)?;
    trace!("SQliteIo started: [{pager:?}].");
    trace!("Starting SqlitePager...");
    let runtime = SqliteRuntime::start(pager)?;
    trace!("SqlitePager started: [{runtime:?}].");

    Ok(Self { runtime })
  }

  pub fn runtime(&self) -> &SqliteRuntime {
    &self.runtime
  }

  pub fn runtime_mut(&mut self) -> &mut SqliteRuntime {
    &mut self.runtime
  }
}
