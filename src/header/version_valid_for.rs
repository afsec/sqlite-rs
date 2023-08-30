use super::traits::ParseBytes;
use crate::result::SQLiteResult;
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

impl ParseBytes<&[u8]> for VersionValidFor {
  const NAME: &'static str = "VersionValidFor";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; 4] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
