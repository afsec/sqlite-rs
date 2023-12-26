#![no_std]
#![forbid(unsafe_code, non_ascii_idents)]

use crate::btree::header::BtreePageHeader;
use crate::header::SqliteHeader;
use crate::result::SQLiteResult;
use crate::traits::ParseBytes;

#[cfg(feature = "std")]
extern crate std;

pub mod btree;
pub mod header;
pub mod result;
pub mod traits;
#[macro_use]
pub mod macros;

// #[cfg(test)]
// mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct SQLiteDatabase {
  mode: Mode,
  header: SqliteHeader,
  // pages: &'a [u8],
  btree_page_header: BtreePageHeader,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
  InMemoryNoStd,
  Std,
}
impl SQLiteDatabase {
  const MINIMUM_USABLE_SIZE: usize = 480; // TODO: TBD
  pub const MINIMUM_SIZE: usize =
    SqliteHeader::LENGTH_BYTES + Self::MINIMUM_USABLE_SIZE;

  pub fn new_in_memory(bytes: &[u8]) -> SQLiteResult<Self> {
    let mode = Mode::InMemoryNoStd;
    let header = SqliteHeader::try_from(&bytes[0..=99])?;

    let btree_page_header = BtreePageHeader::parse_bytes(&bytes[100..])?;

    let database = SQLiteDatabase {
      mode,
      header,
      btree_page_header,
      // pages,
    };

    Ok(database)
  }

  #[cfg(not(feature = "std"))]
  pub fn new() {
    todo!()
  }

  pub fn mode(&self) -> &Mode {
    &self.mode
  }

  pub fn header(&self) -> &SqliteHeader {
    &self.header
  }

  pub fn btree_page_header(&self) -> &BtreePageHeader {
    &self.btree_page_header
  }
}
