use anyhow::bail;

use crate::result::SQLiteError;

use super::page_size::PageSize;

///  SQLite has the ability to set aside a small number of extra bytes at the
/// end of every page for use by extensions. These extra bytes are used, for
/// example, by the SQLite Encryption Extension to store a nonce and/or
/// cryptographic checksum associated with each page. The "reserved space" size
/// in the 1-byte integer at offset 20 is the number of bytes of space at the
/// end of each page to reserve for extensions. **This value is usually 0.** *The
/// value can be odd.*
///
///  The **"usable size"** of a database page is the page size specified by the
/// 2-byte integer at offset 16 in the header less the "reserved" space size
/// recorded in the 1-byte integer at offset 20 in the header. The usable size
/// of a page might be an odd number.
///
/// However, *the usable size is not allowed to be less than `480`*. In other words, if the page size is 512, then the
/// reserved space size cannot exceed 32.
#[derive(Debug)]
pub struct ReservedBytesPerPage(u8);
impl ReservedBytesPerPage {
  pub fn get(&self) -> u8 {
    self.0
  }
}
impl TryFrom<(&PageSize, u8)> for ReservedBytesPerPage {
  type Error = SQLiteError;

  fn try_from(
    (pagesize, reserved_bytes): (&PageSize, u8),
  ) -> Result<Self, Self::Error> {
    if **pagesize == 512 && reserved_bytes > 32 {
      bail!("Usable size is not allowed be less than 480")
    }

    Ok(Self(reserved_bytes))
  }
}
