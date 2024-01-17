use crate::result::{SqliteError, SqliteResult};
use crate::traits::SqliteRawIo;
use crate::{error, trace};
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
  mode: SqliteUriFileMode,
}

impl SqliteUri {
  pub fn path(&self) -> &PathBuf {
    &self.path
  }
}
impl FromStr for SqliteUri {
  type Err = SqliteError;

  fn from_str(uri_str: &str) -> Result<Self, Self::Err> {
    let mut iter_uri = uri_str.split("://");
    let maybe_schema = iter_uri.next();
    let maybe_path = iter_uri.next();
    match (maybe_schema, maybe_path) {
      (Some(_), Some(path_str)) => {
        let mut iter_path = path_str.split('?');
        let file_path = iter_path
          .next()
          .ok_or(SqliteError::Custom("Filepath not defined".into()))
          .map_err(|err| {
            error!("{err}");
            err
          })?;
        let mode = iter_path
          .next()
          .and_then(|s| {
            trace!("Trying to parse mode [{s}]");
            s.parse::<SqliteUriFileMode>().ok()
          })
          .unwrap_or_default();
        trace!("{mode:?}");
        let file_path = PathBuf::from_str(file_path).unwrap();
        trace!("{file_path:?}");
        let path = if mode == SqliteUriFileMode::ReadWriteCreate {
          create_file(&file_path)?;
          file_path
        } else {
          file_path.canonicalize().map_err(|err| {
            error!("Error on open file [{uri_str}]: [{err}].");
            error!("Hint: You can change mode to `?mode=rwc` or check you file path.");
            SqliteError::Custom("Error on parsing file path".into())
          })?
        };
        // TODO: Implement modes
        // if file_path.exists()  .not() {
        //   return Err(Sqlite);
        // }

        Ok(Self {
          uri: uri_str.into(),
          path,
          mode,
        })
      }
      _ => {
        error!("Error on parsing sqlite connection uri[{}]", uri_str);
        Err(SqliteError::Custom(
          "Error on parsing sqlite connection uri".into(),
        ))
      }
    }
  }
}

///  The mode query parameter determines if the new database is opened
/// read-only, read-write, read-write and created if it does not exist, or that
/// the database is a pure in-memory database that never interacts with disk,
/// respectively.
///
/// *Reference:* https://www.sqlite.org/uri.html#urimode
#[derive(Debug, Default, PartialEq, Eq)]
pub enum SqliteUriFileMode {
  ReadOnly,
  #[default]
  ReadWrite,
  ReadWriteCreate,
}

impl FromStr for SqliteUriFileMode {
  type Err = SqliteError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    trace!("impl FromStr for SqliteUriFileMode {s}");
    match s {
      "mode=ro" => Ok(Self::ReadOnly),
      "mode=rw" => Ok(Self::ReadWrite),
      "mode=rwc" => Ok(Self::ReadWriteCreate),
      _ => Err(SqliteError::InvalidFileUriMode),
    }
  }
}

fn create_file(path: &PathBuf) -> SqliteResult<()> {
  let maybe_parent_dir = path.parent();
  maybe_parent_dir
    .map(std::fs::create_dir_all)
    .transpose()?;
  File::create(path)?;
  Ok(())
}
