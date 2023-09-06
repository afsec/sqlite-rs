use super::traits::ParseBytes;
use crate::result::{SQLiteError, SQLiteResult};

/// # Schema format number (4 Bytes)
///
///  The schema format number is a 4-byte big-endian integer at offset 44. The
/// schema format number is similar to the file format read and write version
/// numbers at offsets 18 and 19 except that the schema format number refers to
/// the high-level SQL formatting rather than the low-level b-tree formatting.
/// Four schema format numbers are currently defined:
///
/// - Format 1 is understood by all versions of SQLite back to version 3.0.0
///   (2004-06-18).
///
/// - Format 2 adds the ability of rows within the same table to have a varying
///   number of columns, in order to support the ALTER TABLE ... ADD COLUMN
///   functionality. Support for reading and writing format 2 was added in
///   SQLite version 3.1.3 on 2005-02-20.
///
/// - Format 3 adds the ability of extra columns added by
///   ALTER TABLE ... ADD COLUMN to have non-NULL default values. This
///   capability was added in SQLite version 3.1.4 on 2005-03-11.
///
/// - Format 4 causes SQLite to respect the DESC keyword on index declarations.
///   (The DESC keyword is ignored in indexes for formats 1, 2, and 3.) Format 4
///   also adds two new boolean record type values (serial types 8 and 9).
///   Support for format 4 was added in SQLite 3.3.0 on 2006-01-10.
///
///  New database files created by SQLite use format 4 by default. The
/// legacy_file_format pragma can be used to cause SQLite to create new database
/// files using format 1. The format version number can be made to default to 1
/// instead of 4 by setting SQLITE_DEFAULT_FILE_FORMAT=1 at compile-time.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum SchemaFormat {
  Format1,
  Format2,
  Format3,
  #[default]
  Format4,
}

impl TryFrom<u32> for SchemaFormat {
  type Error = SQLiteError;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(Self::Format1),
      2 => Ok(Self::Format2),
      3 => Ok(Self::Format3),
      4 => Ok(Self::Format4),
      _ => Err(SQLiteError::Custom(stringify!(
        "Invalid payload for {}",
        Self::NAME
      ))),
    }
  }
}

impl From<&SchemaFormat> for u32 {
  fn from(value: &SchemaFormat) -> Self {
    match value {
      SchemaFormat::Format1 => 1,
      SchemaFormat::Format2 => 2,
      SchemaFormat::Format3 => 3,
      SchemaFormat::Format4 => 4,
    }
  }
}

impl ParseBytes for SchemaFormat {
  const NAME: &'static str = "SchemaFormat";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let value = u32::from_be_bytes(buf);

    value.try_into()
  }
}
