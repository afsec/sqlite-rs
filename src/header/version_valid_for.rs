use crate::traits::ParseBytes;
use crate::VERSION_NUMBER;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # Version-valid-for number (4 Bytes)
///
///  The 4-byte big-endian integer at offset 92 is the value of the change
/// counter when the version number was stored. The integer at offset 92
/// indicates which transaction the version number is valid for and is sometimes
/// called the "version-valid-for number".
///
/// >  The entries at offsets 92 and 96 were added in later version of the
/// > SQLite library.
/// >
/// >  When an older version modifies the file, it will change the change
/// > counter (offset 24), but not adjust the version-valid-for number or the
/// > write library version number. So the library version number is no longer
/// > correct, because a different version last wrote to the file.
/// >
/// >  The version-valid-for number allows a new library to detect this case: if
/// > the change counter and the version-valid-for number do not match, then the
/// > write library version number is outdated, and must be ignored.
/// >
/// >  **Reference:** https://stackoverflow.com/a/45420823
#[derive(Debug)]
pub struct VersionValidFor(u32);
impl Default for VersionValidFor {
  fn default() -> Self {
    Self(*VERSION_NUMBER.get().unwrap_or(&0))
  }
}
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
