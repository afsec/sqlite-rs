use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # In-header database size (4 Bytes)
///
///  The in-header database size is a 4-byte big-endian integer at offset 28
/// into the header stores the size of the database file in pages. If this
/// in-header datasize size is not valid (see the next paragraph), then the
/// database size is computed by looking at the actual size of the database
/// file. Older versions of Sqlite ignored the in-header database size and used
/// the actual file size exclusively. Newer versions of Sqlite use the in-header
/// database size if it is available but fall back to the actual file size if
/// the in-header database size is not valid.
///
///  The in-header database size is only considered to be valid if it is
/// non-zero and if the 4-byte change counter at offset 24 exactly matches the
/// 4-byte version-valid-for number at offset 92. The in-header database size is
/// always valid when the database is only modified using recent versions of
/// Sqlite, versions 3.7.0 (2010-07-21) and later. If a legacy version of Sqlite
/// writes to the database, it will not know to update the in-header database
/// size and so the in-header database size could be incorrect. But legacy
/// versions of Sqlite will also leave the version-valid-for number at offset 92
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
impl_name! {DatabaseFileSizeInPages}

impl ParseBytes for DatabaseFileSizeInPages {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
