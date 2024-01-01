use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # Free page list (8 Bytes) => First(4 Bytes) + TotalPages (4 Bytes)
///  Unused pages in the database file are stored on a freelist.
#[derive(Debug)]
pub struct FreeListPages {
  /// Page number of the first freelist trunk page. (4 Bytes)
  first: FreeListPagesFirstTrunkPage,
  /// Total number of freelist pages. (4 Bytes)
  total: FreeListPagesTotalPages,
}

impl FreeListPages {
  pub fn first(&self) -> &FreeListPagesFirstTrunkPage {
    &self.first
  }

  pub fn total(&self) -> &FreeListPagesTotalPages {
    &self.total
  }
}
impl_name! {FreeListPages}

impl ParseBytes for FreeListPages {
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let first = FreeListPagesFirstTrunkPage::parse_bytes(&bytes[0..=3])?;
    let total = FreeListPagesTotalPages::parse_bytes(&bytes[4..=7])?;

    Ok(Self { first, total })
  }
}

///  FreeListPagesFirstTrunkPage: The 4-byte big-endian integer at offset 32
/// stores the page number of the first page of the freelist, or zero if the
/// freelist is empty.
#[derive(Debug)]
pub struct FreeListPagesFirstTrunkPage(u32);

impl Deref for FreeListPagesFirstTrunkPage {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {FreeListPagesFirstTrunkPage}

impl ParseBytes for FreeListPagesFirstTrunkPage {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;
    let first_page_trunk = u32::from_be_bytes(buf);
    Ok(Self(first_page_trunk))
  }
}

///  FreeListPagesTotalPages: The 4-byte big-endian integer at offset 36
/// stores the total number of pages on the freelist.
#[derive(Debug)]
pub struct FreeListPagesTotalPages(u32);

impl Deref for FreeListPagesTotalPages {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {FreeListPagesTotalPages}

impl ParseBytes for FreeListPagesTotalPages {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;
    let total_pages = u32::from_be_bytes(buf);
    Ok(Self(total_pages))
  }
}
