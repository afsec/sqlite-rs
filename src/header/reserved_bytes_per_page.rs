use core::ops::Deref;

use super::traits::ParseBytes;
use crate::result::SQLiteError;
use alloc::format;

/// # Reserved bytes per page (1 Byte)
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

impl Deref for ReservedBytesPerPage {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ParseBytes<&[u8]> for ReservedBytesPerPage {
  fn struct_name() -> &'static str {
    "ReservedBytesPerPage"
  }

  fn bytes_length() -> usize {
    1
  }

  fn parsing_handler(bytes: &[u8]) -> crate::result::SQLiteResult<Self> {
    let reserved_bytes_per_page = *bytes.first().ok_or(SQLiteError::from(
      format!("Impossible state on parsing {}", Self::struct_name()),
    ))?;

    Ok(Self(reserved_bytes_per_page))
  }
}
/*
  fn parse_bytes(bytes: (&PageSize, u8)) -> crate::result::SQLiteResult<Self> {
    let (pagesize, reserved_bytes) = bytes;
    if **pagesize == 512 && reserved_bytes > 32 {
      bail!("Usable size is not allowed be less than 480")
    } else {
      Ok(Self(reserved_bytes))
    }
  }


*/
