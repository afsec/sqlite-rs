use super::traits::ParseBytes;
use crate::result::{SQLiteError, SQLiteResult};
use core::fmt::Display;

/// # Text encoding(4 Bytes)
///
///  The 4-byte big-endian integer at offset 56 determines the encoding used for
/// all text strings stored in the database. A value of 1 means UTF-8. A value
/// of 2 means UTF-16le. A value of 3 means UTF-16be. No other values are
/// allowed. The sqlite3.h header file defines C-preprocessor macros
/// SQLITE_UTF8 as 1, SQLITE_UTF16LE as 2, and SQLITE_UTF16BE as 3, to use in
/// place of the numeric codes for the text encoding.
#[derive(Debug)]
pub enum DatabaseTextEncoding {
  Utf8,
  Utf16Le,
  Utf16Be,
}

impl From<&DatabaseTextEncoding> for u32 {
  fn from(value: &DatabaseTextEncoding) -> Self {
    match value {
      DatabaseTextEncoding::Utf8 => 1,
      DatabaseTextEncoding::Utf16Le => 2,
      DatabaseTextEncoding::Utf16Be => 3,
    }
  }
}

impl TryFrom<u32> for DatabaseTextEncoding {
  type Error = SQLiteError;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(Self::Utf8),
      2 => Ok(Self::Utf16Le),
      3 => Ok(Self::Utf16Be),
      _ => Err(SQLiteError::msg("Invalid payload for DatabaseTextEncoding")),
    }
  }
}
impl Display for DatabaseTextEncoding {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let number = u32::from(self);
    let name = match self {
      DatabaseTextEncoding::Utf8 => "utf8",
      DatabaseTextEncoding::Utf16Le => "utf16le",
      DatabaseTextEncoding::Utf16Be => "utf16le",
    };
    write!(f, "{number} ({name})")
  }
}
impl ParseBytes for DatabaseTextEncoding {
  const NAME: &'static str = "DatabaseTextEncoding";

  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let value = u32::from_be_bytes(buf);

    value.try_into()
  }
}
