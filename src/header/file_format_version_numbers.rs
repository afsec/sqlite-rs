use super::ParseBytes;
use crate::result::SQLiteResult;
use anyhow::bail;

/// # File format version numbers (2 Bytes)
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
#[derive(Debug)]
pub(super) struct FileFormatVersionNumbers {
  write_version: FileFormatWriteVersion,
  read_version: FileFormatReadVersion,
}
impl ParseBytes<&[u8]> for FileFormatVersionNumbers {
  fn struct_name() -> &'static str {
    "FileFormatVersionNumbers"
  }

  fn valid_size() -> usize {
    2
  }

  fn parse_bytes(input: &[u8]) -> SQLiteResult<Self> {
    let bytes = input;
    Self::check_payload_size(bytes)?;
    let write_version = FileFormatWriteVersion::parse_bytes(bytes[0])?;
    let read_version = FileFormatReadVersion::parse_bytes(bytes[1])?;
    Ok(Self {
      write_version,
      read_version,
    })
  }
}

#[derive(Debug)]
pub(super) enum FileFormatWriteVersion {
  Legacy,
  /// Write-Ahead Log
  ///
  /// Reference: https://www.sqlite.org/wal.html
  WAL,
}

impl ParseBytes<u8> for FileFormatWriteVersion {
  fn struct_name() -> &'static str {
    "FileFormatReadVersion"
  }

  fn valid_size() -> usize {
    1
  }

  fn parse_bytes(one_byte: u8) -> crate::result::SQLiteResult<Self> {
    match one_byte {
      1 => Ok(Self::Legacy),
      2 => Ok(Self::WAL),
      _ => bail!("Invalid payload for FileFormatReadVersion"),
    }
  }
}

#[derive(Debug)]
pub(super) enum FileFormatReadVersion {
  Legacy,
  /// Write-Ahead Log
  ///
  /// Reference: https://www.sqlite.org/wal.html
  WAL,
}
impl ParseBytes<u8> for FileFormatReadVersion {
  fn struct_name() -> &'static str {
    "FileFormatReadVersion"
  }

  fn valid_size() -> usize {
    1
  }

  fn parse_bytes(one_byte: u8) -> crate::result::SQLiteResult<Self> {
    match one_byte {
      1 => Ok(Self::Legacy),
      2 => Ok(Self::WAL),
      _ => bail!("Invalid payload for FileFormatReadVersion"),
    }
  }
}
