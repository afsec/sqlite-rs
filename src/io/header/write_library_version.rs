use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # Write library version number (4 Bytes)
///
///  The 4-byte big-endian integer at offset 96 stores the SQLITE_VERSION_NUMBER
/// value for the Sqlite library that most recently modified the database file.
#[derive(Debug)]
pub struct WriteLibraryVersion(u32);

impl Deref for WriteLibraryVersion {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {WriteLibraryVersion}

impl ParseBytes for WriteLibraryVersion {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
