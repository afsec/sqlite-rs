//! # Btree leaf page
//!
//!  Define the depth of a leaf b-tree to be 1.
//!
//!  A leaf b-tree page has no pointers, but it still uses the cell structure to
//! hold keys for index b-trees or keys and content for table b-trees. Data is
//! also contained in the cell.

use crate::pager::btree::header_fields::{
  FirstFreeBlock, FragmentedFreeBytes, NumberOfCells, RightMostPointer,
  StartofCellContentArea,
};
use crate::result::SqliteResult;
use crate::traits::ParseBytes;

/// #### B-tree Page Header Format
/// | Offset | Size | Description                                              |
/// |--------|------|----------------------------------------------------------|
/// | 0      | 1    |  The one-byte flag at offset 0 indicating the b-tree     |
/// |        |      | page type.                                               |
/// |        |      |     - A value of 2 (0x02) means the page is an interior  |
/// |        |      |       index b-tree page.                                 |
/// |        |      |     - A value of 5 (0x05) means the page is an interior  |
/// |        |      |       table b-tree page.                                 |
/// |        |      |     - A value of 10 (0x0a) means the page is a leaf      |
/// |        |      |       index b-tree page.                                 |
/// |        |      |     - A value of 13 (0x0d) means the page is a leaf      |
/// |        |      |       table b-tree page.                                 |
/// |        |      |     - Any other value for the b-tree page type is an     |
/// |        |      |       error.                                             |
/// | 1      | 2    |  The two-byte integer at offset 1 gives the start of the |
/// |        |      | first freeblock on the page, or is zero if there are no  |
/// |        |      | freeblocks.                                              |
/// | 3      | 2    |  The two-byte integer at offset 3 gives the number of    |
/// |        |      | cells on the page.                                       |
/// | 5      | 2    |  The two-byte integer at offset 5 designates the start of|
/// |        |      | the cell content area. A zero value for this integer is  |
/// |        |      | interpreted as 65536.                                    |
/// | 7      | 1    |  The one-byte integer at offset 7 gives the number of    |
/// |        |      | fragmented free bytes within the cell content area.      |

#[derive(Debug)]
pub(crate) struct BtreePageHeaderLeaf {
  first_freeblock: FirstFreeBlock,
  number_of_cells: NumberOfCells,
  start_of_cell_content_area: StartofCellContentArea,
  fragmented_free_bytes: FragmentedFreeBytes,
}

impl BtreePageHeaderLeaf {
  pub(crate) fn first_freeblock(&self) -> &FirstFreeBlock {
    &self.first_freeblock
  }

  pub(crate) fn number_of_cells(&self) -> &NumberOfCells {
    &self.number_of_cells
  }

  pub(crate) fn start_of_cell_content_area(&self) -> &StartofCellContentArea {
    &self.start_of_cell_content_area
  }
  pub(crate) fn fragmented_free_bytes(&self) -> &FragmentedFreeBytes {
    &self.fragmented_free_bytes
  }
}

impl_name! {BtreePageHeaderLeaf}
impl ParseBytes for BtreePageHeaderLeaf {
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let (first_freeblock, to_parse3) =
      bytes.split_at(FirstFreeBlock::LENGTH_BYTES);

    let first_freeblock = FirstFreeBlock::parse_bytes(first_freeblock)?;
    trace!(
      "Parsed BtreePageHeaderLeaf {{first_freeblock: {first_freeblock:?}}}"
    );
    let (number_of_cells, to_parse5) =
      to_parse3.split_at(NumberOfCells::LENGTH_BYTES);

    let number_of_cells = NumberOfCells::parse_bytes(number_of_cells)?;
    trace!(
      "Parsed BtreePageHeaderLeaf {{number_of_cells: {number_of_cells:?}}}"
    );
    let (start_of_cell_content_area, to_parse7) =
      to_parse5.split_at(StartofCellContentArea::LENGTH_BYTES);

    let start_of_cell_content_area =
      StartofCellContentArea::parse_bytes(start_of_cell_content_area)?;
    trace!(
      "Parsed BtreePageHeaderLeaf {{start_of_cell_content_area: {start_of_cell_content_area:?}}}"
    );
    let (fragmented_free_bytes, _) =
      to_parse7.split_at(FragmentedFreeBytes::LENGTH_BYTES);

    let fragmented_free_bytes =
      FragmentedFreeBytes::parse_bytes(fragmented_free_bytes)?;
    trace!(
        "Parsed BtreePageHeaderLeaf {{fragmented_free_bytes: {fragmented_free_bytes:?}}}"
      );
    Ok(Self {
      first_freeblock,
      number_of_cells,
      start_of_cell_content_area,
      fragmented_free_bytes,
    })
  }
}
