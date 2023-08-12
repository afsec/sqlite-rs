use super::ParseBytes;
use anyhow::bail;
use std::ops::Deref;

/// # Page Size (2 Bytes)
///  The two-byte value beginning at offset 16 determines the page size of the
/// database. For SQLite versions 3.7.0.1 (2010-08-04) and earlier, this value
/// is interpreted as a big-endian integer and must be a power of two between
/// 512 and 32768, inclusive. Beginning with SQLite version 3.7.1 (2010-08-23),
/// a page size of 65536 bytes is supported. The value 65536 will not fit in a
/// two-byte integer, so to specify a 65536-byte page size, the value at offset
/// 16 is 0x00 0x01. This value can be interpreted as a big-endian 1 and
/// thought of as a magic number to represent the 65536 page size. Or one can
/// view the two-byte field as a little endian number and say that it
/// represents the page size divided by 256. These two interpretations of the
/// page-size field are equivalent.
#[derive(Debug)]
pub struct PageSize(u32);
impl PageSize {
  pub fn get(&self) -> u32 {
    self.0
  }
}
impl Deref for PageSize {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ParseBytes<&[u8]> for PageSize {
  fn struct_name() -> &'static str {
    "PageSize"
  }

  fn bytes_length() -> usize {
    2
  }

  fn parsing_handler(input: &[u8]) -> crate::result::SQLiteResult<Self> {
    use std::ops::Not;

    let bytes = input;
    Self::check_payload_size(bytes)?;

    let buf: [u8; 2] = bytes.try_into()?;

    let page_size = u16::from_be_bytes(buf);

    if page_size == 1 {
      Ok(Self(65_536))
    } else {
      if page_size < 512 {
        bail!("Page size [{page_size}] can't be less than 512");
      }
      if page_size.is_power_of_two().not() {
        bail!("Page size must be power of two");
      }

      Ok(Self(page_size.into()))
    }
  }
}
