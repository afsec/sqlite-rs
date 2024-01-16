use crate::header::SAMPLE_HEADER;
use crate::result::{SqliteError, SqliteResult};
use crate::traits::SqliteRawIo;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::str::FromStr;

// #[cfg(test)]
// mod tests;

pub struct SqliteIo {
  mode: SqliteIoMode,
  raw_io: Box<dyn SqliteRawIo>,
}

impl Debug for SqliteIo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SqliteIo")
      .field("mode", &self.mode)
      .finish()
  }
}

// TODO: Sample HEADER (remove)
impl Default for SqliteIo {
  fn default() -> Self {
    let cursor: Box<Cursor<Vec<u8>>> =
      Box::new(Cursor::new(SAMPLE_HEADER.to_vec()));
    let raw_io = cursor as Box<dyn SqliteRawIo>;
    Self {
      mode: SqliteIoMode::InMemory,
      raw_io,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SqliteIoMode {
  InMemory,
  File,
}
impl Display for SqliteIoMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
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
        let raw_io = cursor as Box<dyn SqliteRawIo>;
        Ok(Self { mode, raw_io })
      }

      SqliteIoMode::File => {
        let uri = conn_str.parse::<SqliteUri>()?;
        let file = Box::new(File::open(uri.path())?);
        let raw_io: Box<dyn SqliteRawIo> = file as Box<dyn SqliteRawIo>;
        Ok(Self { mode, raw_io })
      }
    }
  }

  pub fn is_empty(&mut self) -> SqliteResult<bool> {
    if self.raw_io.read(&mut [0u8; 1])? == 0 {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub fn read(&mut self, buf: &mut [u8]) -> SqliteResult<usize> {
    Ok(self.raw_io.read(buf)?)
  }

  pub fn seek(&mut self, pos: u64) -> SqliteResult<u64> {
    Ok(self.raw_io.seek(SeekFrom::Start(pos))?)
  }

  pub fn rewind(&mut self) -> SqliteResult<()> {
    Ok(self.raw_io.rewind()?)
  }
  pub fn stream_position(&mut self) -> SqliteResult<u64> {
    Ok(self.raw_io.stream_position()?)
  }

  pub fn close() -> SqliteResult<()> {
    todo!("Close not yet implemented");
  }

  pub fn mode(&self) -> &SqliteIoMode {
    &self.mode
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
    let mut iter_uri = uri_str.split("://");
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
          uri: uri_str.into(),
          path,
        })
      }
      _ => Err(SqliteError::Custom(
        "Error on parsing sqlite connection uri".into(),
      )),
    }
  }
}
