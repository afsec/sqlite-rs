//!  The b-tree page header is 8 bytes in size for leaf pages and 12 bytes for
//! interior pages. All multibyte values in the page header are big-endian. The
//! b-tree page header is composed of the following fields:

// TODO:
//
// +---+-------+-----------+-----------+-----------+
// | h .       |           |           |           |
// | e .       |           |           |           |
// | a . root  |   page 2  |    ...    |   page N  |
// | d . page  |           |           |           |
// | e .       |           |           |           |
// | r .       |           |           |           |
// +---+-------+-----------+-----------+-----------+
//        ^           ^           ^
// | page size | page size | page size | page size |

pub mod first_freeblock;
pub mod number_of_cells;
pub mod page_type;

use crate::{
  impl_name,
  result::{SQLiteError, SQLiteResult},
  traits::ParseBytes,
};

use self::{
  first_freeblock::BtreePageFirstFreeBlock,
  number_of_cells::BtreePageNumberOfCells, page_type::BtreePageType,
};

/// # B-tree Page Header Format (8 Bytes)
///
/// |Offset | Size  | Description|
/// |-------|-------|------------|
/// |  0    |  1    | The one-byte flag at offset 0 indicating the b-tree page type. |
/// |  1    |  2    | The two-byte integer at offset 1 gives the start of the first freeblock on the page, or is zero if there are no freeblocks. |
/// |  3    |  2    | The two-byte integer at offset 3 gives the number of cells on the page. |
/// |  5    |  2    | The two-byte integer at offset 5 designates the start of the cell content area. A zero value for this integer is interpreted as 65536. |
/// |  7    |  1    | The one-byte integer at offset 7 gives the number of fragmented free bytes within the cell content area. |
/// |  8    |  4    | The four-byte page number at offset 8 is the right-most pointer. This value appears in the header of interior b-tree pages only and is omitted from all other pages. |
#[derive(Debug, PartialEq, Eq)]
pub struct BtreePageHeader {
  page_type: BtreePageType,
  first_freeblock: BtreePageFirstFreeBlock,
  number_of_cells: BtreePageNumberOfCells,
  // btree_header_idx5: u8,
  // btree_header_idx6: u8,
  // btree_header_idx7: u8,
}

impl BtreePageHeader {
  pub fn page_type(&self) -> &BtreePageType {
    &self.page_type
  }

  pub fn first_freeblock(&self) -> &BtreePageFirstFreeBlock {
    &self.first_freeblock
  }

  pub fn number_of_cells(&self) -> &BtreePageNumberOfCells {
    &self.number_of_cells
  }
}

impl_name! {BtreePageHeader}

impl ParseBytes for BtreePageHeader {
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(btree_header: &[u8]) -> SQLiteResult<Self> {
    Ok(Self {
      page_type: BtreePageType::parse_bytes(&[btree_header[0]])?,
      first_freeblock: BtreePageFirstFreeBlock::parse_bytes(&[
        btree_header[1],
        btree_header[2],
      ])?,

      number_of_cells: BtreePageNumberOfCells::parse_bytes(&[
        btree_header[3],
        btree_header[4],
      ])?,
    })
  }
}

impl TryFrom<&[u8]> for BtreePageHeader {
  type Error = SQLiteError;

  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    let parsed = Self::parse_bytes(bytes)?;
    // parsed.validate_parsed()?;
    Ok(parsed)
  }
}
