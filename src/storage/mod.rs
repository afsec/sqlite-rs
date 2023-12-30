use self::header::SqliteHeader;
use self::pager::SqlitePager;
use crate::result::{SqliteError, SqliteResult};
use core::str::FromStr;
use std::dbg;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::string::String;
use std::vec;
use std::vec::Vec;

pub mod btree;
pub mod header;
pub mod pager;

// #[cfg(test)]
// mod tests;

#[derive(Debug)]
pub struct SqliteStorage {
  mode: SqliteStorageMode,
  pager: SqlitePager,
}

#[derive(Debug)]
pub enum SqliteStorageMode {
  NoStdInMemory,
  InMemory(Cursor<Vec<u8>>),
  File(Cursor<File>),
}

impl FromStr for SqliteStorageMode {
  type Err = SqliteError;

  fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
    let mode = match uri_str.trim() {
      ":memory:" => {
        let inner: Cursor<Vec<u8>> = Cursor::new(vec![]);
        SqliteStorageMode::InMemory(inner)
      }
      raw_path => {
        let uri = raw_path.parse::<SqliteUri>()?;
        let file = Cursor::new(File::open(uri.path())?);
        SqliteStorageMode::File(file)
      }
    };
    Ok(mode)
  }
}
#[derive(Debug)]
pub struct SqliteUri {
  uri: String,
  path: PathBuf,
}

impl SqliteUri {
  pub fn path(&self) -> &PathBuf {
    &self.path
  }
}
impl FromStr for SqliteUri {
  type Err = SqliteError;

  fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
    
    // TODO: generate tests for: sqlite:///home/user/db.sqlite3
    let mut iter_uri = uri_str.split("://").into_iter();
    let maybe_schema = iter_uri.next();
    let maybe_path = iter_uri.next();
    match (maybe_schema, maybe_path) {
      (Some(_), Some(path_str)) => {
        let path = PathBuf::from_str(path_str)
          .unwrap()
          .canonicalize()
          .map_err(|_| SqliteError::Custom("Error on parsing file path".into()))?;

        Ok(Self {
          uri: uri_str.to_string(),
          path,
        })
      }
      _ => Err(SqliteError::Custom(
        "Error on parsing sqlite connection uri".into(),
      )),
    }
  }
}
impl<'uri> SqliteStorage {
  const MINIMUM_USABLE_SIZE: usize = 480; // TODO: TBD
  pub const MINIMUM_SIZE: usize =
    SqliteHeader::LENGTH_BYTES + Self::MINIMUM_USABLE_SIZE;

  // pub fn new_in_memory(bytes: &[u8]) -> SqliteResult<Self> {
  //   let mode = Mode::InMemoryNoStd;
  //   let header = SqliteHeader::try_from(&bytes[0..=99])?;

  //   let btree_page_header = BtreePageHeader::parse_bytes(&bytes[100..])?;

  //   let database = SqliteStorage {
  //     mode,
  //     header,
  //     btree_page_header,
  //     // pages,
  //   };

  //   Ok(database)
  // }

  // #[cfg(not(feature = "std"))]
  // pub fn new() {
  //   todo!()
  // }

  pub fn create() -> SqliteResult<Self> {
    todo!();
  }

  pub fn open(conn_str: &'uri str) -> SqliteResult<Self> {
    let mode = conn_str.parse::<SqliteStorageMode>()?;
    dbg!(conn_str, &mode);
    Ok(Self {
      mode,
      pager: SqlitePager {},
    })
  }
  pub fn close() -> SqliteResult<()> {
    todo!();
  }
}
