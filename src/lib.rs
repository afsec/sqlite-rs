#![forbid(unsafe_code, non_ascii_idents)]

use crate::result::SqliteResult;
use crate::runtime::SqliteRuntime;
use crate::storage::SqliteStorage;

pub mod result;
pub mod runtime;
pub mod storage;
pub mod traits;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct SqliteDatabase;

impl<'uri> SqliteDatabase {
  pub fn open(conn_str: &'uri str) -> SqliteResult<SqliteConnection> {
    SqliteConnection::open(conn_str)
  }
  pub fn close(self) -> SqliteResult<()> {
    Ok(())
  }
}

#[derive(Debug)]
pub struct SqliteConnection {
  runtime: SqliteRuntime,
  storage: SqliteStorage,
}
impl<'uri> SqliteConnection {
  pub fn open(conn_str: &'uri str) -> SqliteResult<SqliteConnection> {
    let runtime = SqliteRuntime::start()?;
    let storage = SqliteStorage::open(conn_str)?;
    let db = SqliteConnection { runtime, storage };
    Ok(db)
  }
}
