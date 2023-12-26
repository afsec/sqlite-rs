#![forbid(unsafe_code, non_ascii_idents)]

use crate::io::SqliteIo;
use crate::pager::SqlitePager;
use crate::result::SqliteResult;
use crate::runtime::SqliteRuntime;

pub mod header;
pub mod io;
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
  // TODO: Implement type state builder
}
impl SqliteConnection {
  pub fn open(conn_str: impl AsRef<str>) -> SqliteResult<Self> {
    let io = SqliteIo::open(conn_str)?;
    let pager = SqlitePager::connect(io)?;
    let runtime = SqliteRuntime::start(pager)?;

    Ok(Self { runtime })
  }
  #[cfg(all(debug_assertions, test))]
  pub fn open_sample() -> SqliteResult<Self> {
    let io = SqliteIo::default();
    let pager = SqlitePager::connect(io)?;
    let runtime = SqliteRuntime::start(pager)?;

    Ok(Self { runtime })
  }

    pub fn runtime(&self) -> &SqliteRuntime {
        &self.runtime
    }
}
