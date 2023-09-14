use super::traits::ParseBytes;
use crate::{impl_name, result::SQLiteResult};
use core::ops::Deref;

/// # User version number (4 Bytes)
///
///  The 4-byte big-endian integer at offset 60 is the user version which is set
/// and queried by the user_version pragma. The user version is not used by
/// SQLite.
#[derive(Debug)]
pub struct UserVersion(u32);

impl Deref for UserVersion {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl_name! {UserVersion}
impl ParseBytes for UserVersion {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let value = u32::from_be_bytes(buf);

    Ok(Self(value))
  }
}
