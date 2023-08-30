use super::traits::ParseBytes;
use crate::result::SQLiteResult;
use core::ops::Deref;

/// # In-header database size (4 Bytes)
/// 
///  The in-header database size is a 4-byte big-endian integer at offset 28
/// into the header stores the size of the database file in pages. If this
/// in-header datasize size is not valid (see the next paragraph), then the
/// database size is computed by looking at the actual size of the database
/// file. Older versions of SQLite ignored the in-header database size and used
/// the actual file size exclusively. Newer versions of SQLite use the in-header
/// database size if it is available but fall back to the actual file size if
/// the in-header database size is not valid.
///
///  The in-header database size is only considered to be valid if it is
/// non-zero and if the 4-byte change counter at offset 24 exactly matches the
/// 4-byte version-valid-for number at offset 92. The in-header database size is
/// always valid when the database is only modified using recent versions of
/// SQLite, versions 3.7.0 (2010-07-21) and later. If a legacy version of SQLite
/// writes to the database, it will not know to update the in-header database
/// size and so the in-header database size could be incorrect. But legacy
/// versions of SQLite will also leave the version-valid-for number at offset 92
/// unchanged so it will not match the change-counter. Hence, invalid in-header
/// database sizes can be detected (and ignored) by observing when the
/// change-counter does not match the version-valid-for number.
#[derive(Debug)]
pub struct DatabaseFileSizeInPages(u32);

impl Deref for DatabaseFileSizeInPages {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ParseBytes<&[u8]> for DatabaseFileSizeInPages {
  const NAME: &'static str = "DatabaseFileSizeInPages";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; 4] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
