use crate::traits::ParseBytes;
use crate::{
  impl_name,
  result::{SqliteError, SqliteResult},
};
use core::ops::Deref;

/// # Page Size (2 Bytes)
///
///  The two-byte value beginning at offset 16 determines the page size of the
/// database. For Sqlite versions 3.7.0.1 (2010-08-04) and earlier, this value
/// is interpreted as a big-endian integer and must be a power of two between
/// 512 and 32768, inclusive. Beginning with Sqlite version 3.7.1 (2010-08-23),
/// a page size of 65536 bytes is supported. The value 65536 will not fit in a
/// two-byte integer, so to specify a 65536-byte page size, the value at offset
/// 16 is 0x00 0x01. This value can be interpreted as a big-endian 1 and
/// thought of as a magic number to represent the 65536 page size. Or one can
/// view the two-byte field as a little endian number and say that it
/// represents the page size divided by 256. These two interpretations of the
/// page-size field are equivalent.
#[derive(Debug)]
pub struct PageSize(u32);

impl Deref for PageSize {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {PageSize}

impl ParseBytes for PageSize {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    use core::ops::Not;

    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let page_size = u16::from_be_bytes(buf);

    if page_size == 1 {
      Ok(Self(65_536))
    } else {
      if page_size < 512 {
        return Err(SqliteError::Custom("PageSize can't be less than 512".into()));
      }
      if page_size.is_power_of_two().not() {
        return Err(SqliteError::Custom("PageSize must be power of two".into()));
      }

      Ok(Self(page_size.into()))
    }
  }
}
