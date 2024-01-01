#![forbid(unsafe_code, non_ascii_idents)]

use io::SqliteIo;

use crate::io::SqliteIoMode;
use crate::pager::SqlitePager;
use crate::result::SqliteResult;
use crate::runtime::SqliteRuntime;

pub mod io;
pub mod pager;
pub mod result;
pub mod runtime;
pub mod traits;
#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

// #[derive(Debug)]
// pub struct SqliteDatabase;

// impl<'uri> SqliteDatabase {
//   pub fn open(conn_str: &'uri str) -> SqliteResult<SqliteConnection> {
//     SqliteConnection::open(conn_str)
//   }
//   pub fn close(self) -> SqliteResult<()> {
//     Ok(())
//   }
// }

#[derive(Debug)]
pub struct SqliteDatabase {
  runtime: SqliteRuntime,
  pager: SqlitePager,
  io: SqliteIoMode,
}
impl SqliteDatabase {
  pub fn open(conn_str: impl AsRef<str>) -> SqliteResult<SqliteConnection> {
    let io = SqliteIo::open(conn_str);
    let runtime = SqliteRuntime::start()?;
    // let storage = SqliteStorage::open(conn_str)?;
    // let db = SqliteConnection { runtime, storage };
    let conn = SqliteConnection;
    Ok(conn)
  }
}

#[derive(Debug)]
// pub struct SqliteConnection(SqliteDatabase);
pub struct SqliteConnection;
