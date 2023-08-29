use core::ops::Deref;
use crate::result::SQLiteResult;
use super::traits::ParseBytes;

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

impl ParseBytes<&[u8]> for FreeListPages {
  const NAME: &'static str = "FreeListPages";
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
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

impl ParseBytes<&[u8]> for FreeListPagesFirstTrunkPage {
  const NAME: &'static str = "FreeListPagesFirstTrunkPage";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; 4] = bytes.try_into()?;
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

impl ParseBytes<&[u8]> for FreeListPagesTotalPages {
  const NAME: &'static str = "FreeListPagesTotalPages";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; 4] = bytes.try_into()?;
    let total_pages = u32::from_be_bytes(buf);
    Ok(Self(total_pages))
  }
}
