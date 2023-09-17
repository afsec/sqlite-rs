use crate::traits::{Name, ParseBytes};
use crate::{field_parsing_error, impl_name, result::SQLiteResult};
use core::fmt::Display;

/// # File format version numbers (2 Bytes)
///
///  The file format write version and file format read version at offsets 18
/// and 19 are intended to allow for enhancements of the file format in future
/// versions of SQLite. In current versions of SQLite, both of these values
/// are:
///   - `1` for rollback journalling modes; and
///   - `2` for WAL journalling mode.
///
///  If a version of SQLite coded to the current file format specification
/// encounters a database file where the read version is 1 or 2 but the write
/// version is greater than 2, then the database file must be treated as
/// read-only. If a database file with a read version greater than 2 is
/// encountered, then that database cannot be read or written.
#[derive(Debug, PartialEq,Eq)]
pub struct FileFormatVersionNumbers {
  /// File format write version. 1 for legacy; 2 for WAL.
  write_version: FileFormatWriteVersion,
  /// File format read version. 1 for legacy; 2 for WAL.
  read_version: FileFormatReadVersion,
}

impl FileFormatVersionNumbers {
  pub fn write_version(&self) -> &FileFormatWriteVersion {
    &self.write_version
  }

  pub fn read_version(&self) -> &FileFormatReadVersion {
    &self.read_version
  }
}
impl_name! {FileFormatVersionNumbers}
impl ParseBytes for FileFormatVersionNumbers {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let write_version = FileFormatWriteVersion::parsing_handler(&[bytes[0]])?;
    let read_version = FileFormatReadVersion::parsing_handler(&[bytes[1]])?;
    Ok(Self {
      write_version,
      read_version,
    })
  }
}

#[derive(Debug, PartialEq,Eq)]
pub enum FileFormatWriteVersion {
  Legacy,
  /// Write-Ahead Log
  ///
  /// Reference: https://www.sqlite.org/wal.html
  WAL,
}

impl From<&FileFormatWriteVersion> for u8 {
  fn from(value: &FileFormatWriteVersion) -> Self {
    match value {
      FileFormatWriteVersion::Legacy => 1,
      FileFormatWriteVersion::WAL => 2,
    }
  }
}

impl_name! {FileFormatWriteVersion}

impl ParseBytes for FileFormatWriteVersion {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let one_byte = *bytes.first().ok_or(field_parsing_error! {Self::NAME})?;
    match one_byte {
      1 => Ok(Self::Legacy),
      2 => Ok(Self::WAL),
      _ => Err(field_parsing_error! {Self::NAME}),
    }
  }
}

impl Display for FileFormatWriteVersion {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", u8::from(self))
  }
}

#[derive(Debug, PartialEq,Eq)]
pub enum FileFormatReadVersion {
  Legacy,
  /// Write-Ahead Log
  ///
  /// Reference: https://www.sqlite.org/wal.html
  WAL,
}

impl From<&FileFormatReadVersion> for u8 {
  fn from(value: &FileFormatReadVersion) -> Self {
    match value {
      FileFormatReadVersion::Legacy => 1,
      FileFormatReadVersion::WAL => 2,
    }
  }
}

impl_name! {FileFormatReadVersion}

impl ParseBytes for FileFormatReadVersion {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let one_byte = *bytes.first().ok_or(field_parsing_error! {Self::NAME})?;
    match one_byte {
      1 => Ok(Self::Legacy),
      2 => Ok(Self::WAL),
      _ => Err(field_parsing_error! {Self::NAME}),
    }
  }
}

impl Display for FileFormatReadVersion {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", u8::from(self))
  }
}
