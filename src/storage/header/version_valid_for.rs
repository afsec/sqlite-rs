use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # Version-valid-for number (4 Bytes)
///
///  The 4-byte big-endian integer at offset 92 is the value of the change
/// counter when the version number was stored. The integer at offset 92
/// indicates which transaction the version number is valid for and is sometimes
/// called the "version-valid-for number".
#[derive(Debug)]
pub struct VersionValidFor(u32);

impl Deref for VersionValidFor {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl_name! {VersionValidFor}
impl ParseBytes for VersionValidFor {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
