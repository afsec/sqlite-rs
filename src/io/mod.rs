use crate::result::{SqliteError, SqliteResult};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;

pub mod btree;
pub mod header;

// #[cfg(test)]
// mod tests;

pub struct SqliteIo {
  mode: SqliteIoMode,
  // InMemory(Cursor<Vec<u8>>),
  // File(Cursor<File>),
  reader: Box<dyn Read>,
}

impl Debug for SqliteIo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SqliteIo")
      .field("mode", &self.mode)
      .field("cursor", &"Box<dyn BufRead>")
      .finish()
  }
}

#[derive(Debug)]
pub enum SqliteIoMode {
  InMemory,
  File,
}

impl FromStr for SqliteIoMode {
  type Err = SqliteError;

  fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
    let mode = match uri_str.trim() {
      ":memory:" => SqliteIoMode::InMemory,
      _ => SqliteIoMode::File,
    };
    Ok(mode)
  }
}
impl SqliteIo {
  pub fn open(input: impl AsRef<str>) -> SqliteResult<Self> {
    let conn_str = input.as_ref();
    let mode = conn_str.parse::<SqliteIoMode>()?;
    match mode {
      SqliteIoMode::InMemory => {
        let cursor: Box<Cursor<Vec<u8>>> = Box::new(Cursor::new(vec![]));
        let reader = cursor as Box<dyn Read>;
        Ok(Self { mode, reader })
      }

      SqliteIoMode::File => {
        let uri = conn_str.parse::<SqliteUri>()?;
        let file = Box::new(File::open(uri.path())?);
        let reader = file as Box<dyn Read>;
        Ok(Self { mode, reader })
      }
    }
  }
  pub fn close() -> SqliteResult<()> {
    todo!();
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
          .map_err(|_| {
            SqliteError::Custom("Error on parsing file path".into())
          })?;

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

// impl<'uri> SqliteIoMode {
//   const MINIMUM_USABLE_SIZE: usize = 480; // TODO: TBD
//   pub const MINIMUM_SIZE: usize =
//     SqliteHeader::LENGTH_BYTES + Self::MINIMUM_USABLE_SIZE;

// pub fn new_in_memory(bytes: &[u8]) -> SqliteResult<Self> {
//   let mode = Mode::InMemoryNoStd;
//   let header = SqliteHeader::try_from(&bytes[0..=99])?;

//   let btree_page_header = BtreePageHeader::parse_bytes(&bytes[100..])?;

//   let database = SqliteIoMode {
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

//   pub fn create() -> SqliteResult<Self> {
//     todo!();
//   }

//   pub fn open(conn_str: &'uri str) -> SqliteResult<Self> {
//     let mode = conn_str.parse::<SqliteIoMode>()?;
//     dbg!(conn_str, &mode);
//     Ok(Self {
//       mode,
//       pager: SqlitePager {},
//     })
//   }
//   pub fn close() -> SqliteResult<()> {
//     todo!();
//   }
// }
