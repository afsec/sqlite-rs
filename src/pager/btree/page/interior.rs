//! # Btree interior page
//!
//!  The number of keys on an interior b-tree page, K, is almost always at least
//! 2 and is usually much more than 2. The only exception is when page 1 is an
//! interior b-tree page. Page 1 has 100 fewer bytes of storage space available,
//! due to the presence of the database header at the beginning of that page,
//! and so sometimes (rarely) if page 1 is an interior b-tree page, it can end
//! up holding just a a single key. In all other cases, K is 2 or more. The
//! upper bound on K is as many keys as will fit on the page. Large keys on
//! index b-trees are split up into overflow pages so that no single key uses
//! more than one fourth of the available storage space on the page and hence
//! every internal page is able to store at least 4 keys. The integer keys of
//! table b-trees are never large enough to require overflow, so key overflow
//! only occurs on index b-trees.
//!
//!  In an interior b-tree page, the pointers and keys logically alternate with
//! a pointer on both ends. (The previous sentence is to be understood
//! conceptually - the actual layout of the keys and pointers within the page is
//! more complicated and will be described in the sequel.) All keys within the
//! same page are unique and are logically organized in ascending order from
//! left to right. (Again, this ordering is logical, not physical. The actual
//! location of keys within the page is arbitrary.) For any key X, pointers to
//! the left of a X refer to b-tree pages on which all keys are less than or
//! equal to X. Pointers to the right of X refer to pages where all keys are
//! greater than X.
//!
//!  The depth of any interior b-tree to be one more than the maximum depth of
//! any of its children. In a well-formed database, all children of an interior
//! b-tree have the same depth.
//!
//!  Within an interior b-tree page, each key and the pointer to its immediate
//! left are combined into a structure called a "cell". The right-most pointer
//! is held separately.

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
/// | 8      | 4    |  The four-byte page number at offset 8 is the right-most |
/// |        |      | pointer. This value appears in the header of interior    |
/// |        |      | b-tree pages only and is omitted from all other pages.   |

#[derive(Debug)]
pub(crate) struct BtreePageHeaderInterior {
  first_freeblock: FirstFreeBlock,
  number_of_cells: NumberOfCells,
  start_of_cell_content_area: StartofCellContentArea,
  fragmented_free_bytes: FragmentedFreeBytes,
  right_most_pointer: RightMostPointer,
}

impl BtreePageHeaderInterior {
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

  pub(crate) fn right_most_pointer(&self) -> &RightMostPointer {
    &self.right_most_pointer
  }
}

impl_name! {BtreePageHeaderInterior}
impl ParseBytes for BtreePageHeaderInterior {
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let (first_freeblock, to_parse3) =
      bytes.split_at(FirstFreeBlock::LENGTH_BYTES);

    let first_freeblock = FirstFreeBlock::parse_bytes(first_freeblock)?;
    trace!(
      "Parsed BtreePageHeaderInterior {{first_freeblock: {first_freeblock:?}}}"
    );
    let (number_of_cells, to_parse5) =
      to_parse3.split_at(NumberOfCells::LENGTH_BYTES);

    let number_of_cells = NumberOfCells::parse_bytes(number_of_cells)?;
    trace!(
      "Parsed BtreePageHeaderInterior {{number_of_cells: {number_of_cells:?}}}"
    );
    let (start_of_cell_content_area, to_parse7) =
      to_parse5.split_at(StartofCellContentArea::LENGTH_BYTES);

    let start_of_cell_content_area =
      StartofCellContentArea::parse_bytes(start_of_cell_content_area)?;
    trace!(
      "Parsed BtreePageHeaderInterior {{start_of_cell_content_area: {start_of_cell_content_area:?}}}"
    );
    let (fragmented_free_bytes, _) =
      to_parse7.split_at(FragmentedFreeBytes::LENGTH_BYTES);

    let fragmented_free_bytes =
      FragmentedFreeBytes::parse_bytes(fragmented_free_bytes)?;
    trace!(
        "Parsed BtreePageHeaderInterior {{fragmented_free_bytes: {fragmented_free_bytes:?}}}"
      );
    Ok(Self {
      first_freeblock,
      number_of_cells,
      start_of_cell_content_area,
      fragmented_free_bytes,
      right_most_pointer: todo!(),
    })
  }
}
