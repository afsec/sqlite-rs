use super::traits::ParseBytes;
use crate::result::SQLiteResult;
use core::ops::Deref;

/// # Write library version number (4 Bytes)
///
///  The 4-byte big-endian integer at offset 96 stores the SQLITE_VERSION_NUMBER
/// value for the SQLite library that most recently modified the database file.
#[derive(Debug)]
pub struct WriteLibraryVersion(u32);

impl Deref for WriteLibraryVersion {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ParseBytes for WriteLibraryVersion {
  const NAME: &'static str = "WriteLibraryVersion";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
