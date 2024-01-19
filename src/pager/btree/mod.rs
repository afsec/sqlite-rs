//! # B-tree Pages
//!
//!  The b-tree algorithm provides key/data storage with unique and ordered keys
//! on page-oriented storage devices. For background information on b-trees, see
//! Knuth, The Art Of Computer Programming, Volume 3 "Sorting and Searching",
//! pages 471-479. Two variants of b-trees are used by SQLite. "Table b-trees"
//! use a 64-bit signed integer key and store all data in the leaves.
//! "Index b-trees" use arbitrary keys and store no data at all.
//!
//!  **Reference:** https://www.sqlite.org/fileformat2.html#b_tree_pages

pub(crate) mod cell;
pub(crate) mod header_fields;
pub(crate) mod page;
pub(crate) mod page_type;

use self::header_fields::FirstFreeBlock;
use self::header_fields::{
  FragmentedFreeBytes, NumberOfCells, RightMostPointer, StartofCellContentArea,
};
use self::page::interior::BtreePageHeaderInterior;
use self::page::leaf::BtreePageHeaderLeaf;
use self::page_type::BtreePageType;
use crate::result::{BtreeError, BtreeErrorParsing, SqliteError, SqliteResult};
use crate::traits::ParseBytes;

#[derive(Debug)]
pub(crate) enum BtreePageHeader {
  Interior(BtreePageHeaderInterior),
  Leaf(BtreePageHeaderLeaf),
}
impl BtreePageHeader {
  pub(crate) fn length_bytes(&self) -> usize {
    match self {
      BtreePageHeader::Interior(_) => BtreePageHeaderInterior::LENGTH_BYTES,
      BtreePageHeader::Leaf(_) => BtreePageHeaderLeaf::LENGTH_BYTES,
    }
  }

  pub(crate) fn first_freeblock(&self) -> &FirstFreeBlock {
    match self {
      BtreePageHeader::Interior(header) => header.first_freeblock(),
      BtreePageHeader::Leaf(header) => header.first_freeblock(),
    }
  }
  pub(crate) fn number_of_cells(&self) -> &NumberOfCells {
    match self {
      BtreePageHeader::Interior(header) => header.number_of_cells(),
      BtreePageHeader::Leaf(header) => header.number_of_cells(),
    }
  }
  pub(crate) fn start_of_cell_content_area(&self) -> &StartofCellContentArea {
    match self {
      BtreePageHeader::Interior(header) => header.start_of_cell_content_area(),
      BtreePageHeader::Leaf(header) => header.start_of_cell_content_area(),
    }
  }
  pub(crate) fn fragmented_free_bytes(&self) -> &FragmentedFreeBytes {
    match self {
      BtreePageHeader::Interior(header) => header.fragmented_free_bytes(),
      BtreePageHeader::Leaf(header) => header.fragmented_free_bytes(),
    }
  }
  pub(crate) fn right_most_pointer(&self) -> Option<&RightMostPointer> {
    match self {
      BtreePageHeader::Interior(header) => Some(header.right_most_pointer()),
      BtreePageHeader::Leaf(header) => None,
    }
  }
  pub(crate) fn parsing_handler(
    page_type: &BtreePageType,
    bytes: &[u8],
  ) -> SqliteResult<Self> {
    let page_kind = match page_type {
      BtreePageType::InteriorIndex | BtreePageType::InteriorTable => {
        BtreePageHeader::Interior(BtreePageHeaderInterior::parse_bytes(bytes)?)
      }
      BtreePageType::LeafIndex | BtreePageType::LeafTable => {
        BtreePageHeader::Leaf(BtreePageHeaderLeaf::parse_bytes(bytes)?)
      }
    };
    Ok(page_kind)
  }
}

/// # BtreePageCellPointerArray
///
///  The cell pointer array of a b-tree page immediately follows the b-tree page
/// header. Let K be the number of cells on the btree. The cell pointer array
/// consists of K 2-byte integer offsets to the cell contents. The cell pointers
/// are arranged in key order with left-most cell (the cell with the smallest
/// key) first and the right-most cell (the cell with the largest key) last.
///
///  Cell content is stored in the cell content region of the b-tree page.
/// SQLite strives to place cells as far toward the end of the b-tree page as it
/// can, in order to leave space for future growth of the cell pointer array.The
/// area in between the last cell pointer array entry and the beginning of the
/// first cell is the unallocated region.
///
///  If a page contains no cells (which is only possible for a root page of a
/// table that contains no rows) then the offset to the cell content area will
/// equal the page size minus the bytes of reserved space. If the database uses
/// a 65536-byte page size and the reserved space is zero (the usual value for
/// reserved space) then the cell content offset of an empty page wants to be
/// 65536. However, that integer is too large to be stored in a 2-byte unsigned
/// integer, so a value of 0 is used in its place.
#[derive(Debug)]
pub(crate) struct BtreePageCellPointerArray(Vec<u16>);
impl BtreePageCellPointerArray {
  pub(crate) fn parsing_handler(
    header: &BtreePageHeader,
    bytes: &[u8],
  ) -> SqliteResult<Self> {
    println!("raw_cell_pointers {bytes:02x?}");
    let number_of_cells = usize::from(header.number_of_cells().into_inner());
    let number_of_bytes_to_parse = usize::from(number_of_cells) * 2;
    if bytes.len() != number_of_bytes_to_parse {
      return Err(SqliteError::Btree(BtreeError::Parsing(
        BtreeErrorParsing::Custom(
          "Invalid bytes length to parse number_of_cells".into(),
        ),
      )));
    }

    let mut inner = vec![];
    for idx in (0..number_of_bytes_to_parse).step_by(2) {
      let buf = [bytes[idx], bytes[idx + 1]];
      // dbg!(buf);
      let cell_offset = u16::from_le_bytes(buf);
      inner.push(cell_offset);
    }
    Ok(Self(inner))
  }
}

///////////////////////////

#[derive(Debug)]
pub enum BtreePageLevel {
  Root(BtreePageRoot),
  Child(BtreePageChild),
}

#[derive(Debug)]
pub struct BtreePageRoot {
  child: BtreePgnu,
}

#[derive(Debug)]
pub struct BtreePageChild {
  parent: BtreePgnu,
  page_type: BtreePageType,
  cell: BtreeCell,
}

#[derive(Debug)]
/// ## Btree page number
///
/// **Related to:**  `crate::header::DatabaseFileSizeInPages`
pub struct BtreePgnu(u32);

#[derive(Debug)]
pub enum BtreeCell {
  /// ### Table B-Tree Cell:
  /// - Represents a single row of data in a table.
  /// - Contains the actual data for the row.
  Table,
  /// ### Index B-Tree Cell:
  /// - Represents a single entry in an index.
  /// - Contains the indexed value and a reference to the corresponding table
  ///   row.
  Index,
}

/*
#[derive(Debug)]
pub enum BtreePageKind {
  /// ### Table B-Tree Interior Page:
  /// - Contains pointers to other pages in the B-tree structure.
  /// - Used in the internal organization of tables.
  TableInterior,
  /// ### Table B-Tree Leaf Page:
  /// - Contains actual data or references to table rows.
  /// - Each entry in the leaf page represents a row in the table.
  TableLeaf,
  /// ### Index B-Tree Interior Page:
  /// - Similar to table B-tree interior pages but used for index structures.
  /// - Contains pointers to other pages in the B-tree structure.
  IndexInterior,
  /// ### Index B-Tree Leaf Page:
  /// - Contains key values and references to rows in the indexed table.
  /// - Each entry in the leaf page represents an indexed value and a reference
  ///   to the corresponding table row.
  IndexLeaf,
}
*/
