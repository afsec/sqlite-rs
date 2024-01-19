use crate::{
  result::{BtreeError, BtreeErrorParsing, SqliteError, SqliteResult},
  traits::ParseBytes,
};

use super::{
  page_type::BtreePageType, BtreePageCellPointerArray, BtreePageHeader,
};

pub(crate) mod header;
pub(crate) mod interior;
pub(crate) mod leaf;

///  Every b-tree page has at most one parent b-tree page. A b-tree page without
/// a parent is called a root page. A root b-tree page together with the closure
/// of its children form a complete b-tree. It is possible (and in fact rather
/// common) to have a complete b-tree that consists of a single page that is
/// both a leaf and the root. Because there are pointers from parents to
/// children, every page of a complete b-tree can be located if only the root
/// page is known. Hence, b-trees are identified by their root page number.
///
///  A b-tree page is either a table b-tree page or an index b-tree page. All
/// pages within each complete b-tree are of the same type: either table or
/// index. There is one table b-trees in the database file for each rowid table
/// in the database schema, including system tables such as sqlite_schema. There
/// is one index b-tree in the database file for each index in the schema,
/// including implied indexes created by uniqueness constraints. There are no
/// b-trees associated with virtual tables. Specific virtual table
/// implementations might make use of shadow tables for storage, but those
/// shadow tables will have separate entries in the database schema.WITHOUT
/// ROWID tables use index b-trees rather than a table b-trees, so there is one
/// index b-tree in the database file for each WITHOUT ROWID table. The b-tree
/// corresponding to the sqlite_schema table is always a table b-tree and always
/// has a root page of 1. The sqlite_schema table contains the root page number
/// for every other table and index in the database file.
///
///  Each entry in a table b-tree consists of a 64-bit signed integer key and up
/// to 2147483647 bytes of arbitrary data. (The key of a table b-tree
/// corresponds to the rowid of the SQL table that the b-tree implements.)
/// Interior table b-trees hold only keys and pointers to children. All data is
/// contained in the table b-tree leaves.
///
///  Each entry in an index b-tree consists of an arbitrary key of up to
/// 2147483647 bytes in length and no data.
///
///  Define the "payload" of a cell to be the arbitrary length section of the
/// cell. For an index b-tree, the key is always arbitrary in length and hence
/// the payload is the key. There are no arbitrary length elements in the cells
/// of interior table b-tree pages and so those cells have no payload. Table
/// b-tree leaf pages contain arbitrary length content and so for cells on those
/// pages the payload is the content.
///
///  When the size of payload for a cell exceeds a certain threshold (to be
/// defined later) then only the first few bytes of the payload are stored on
/// the b-tree page and the balance is stored in a linked list of content
/// overflow pages.

/// # Btree page
///  A b-tree page is either an interior page or a leaf page. A leaf page
/// contains keys and in the case of a table b-tree each key has associated
/// data. An interior page contains K keys together with K+1 pointers to child
/// b-tree pages. A "pointer" in an interior b-tree page is just the *32-bit
/// unsigned integer* page number of the child page.
///
/// A b-tree page is divided into regions in the following order:
///
/// 1. The 100-byte database file header (found on page 1 only)
/// 2. The 8 or 12 byte b-tree page header
/// 3. The cell pointer array
/// 4. Unallocated space
/// 5. The cell content area
/// 6. The reserved region.
/// 7. The 100-byte database file header is found only on page 1, which is
///    always a table b-tree page. All other b-tree pages in the database file
///    omit this 100-byte header.
///
///  The reserved region is an area of unused space at the end of every page
/// (except the locking page) that extensions can use to hold per-page
/// information. The size of the reserved region is determined by the one-byte
/// unsigned integer found at an offset of 20 into the database file header. The
/// size of the reserved region is usually zero.
///
///  The b-tree page header is 8 bytes in size for leaf pages and 12 bytes for
/// interior pages. All multibyte values in the page header are big-endian. The
/// b-tree page header is composed of the following fields:
///
#[derive(Debug)]
pub(crate) struct BtreePage {
  page_type: BtreePageType,
  header: BtreePageHeader,
  //TODO
  cell_pointer_array: BtreePageCellPointerArray,
  // TODO
  // level: BtreePageLevel,
}
impl BtreePage {
  pub(crate) fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let (btree_page_type, to_parse_header) =
      bytes.split_at(BtreePageType::LENGTH_BYTES);
    let btree_type_payload =
      btree_page_type
        .get(0)
        .ok_or(SqliteError::Btree(BtreeError::Parsing(
          BtreeErrorParsing::HeaderInvalidPayloadForPageType,
        )))?;

    let page_type = BtreePageType::parse_bytes(&[*btree_type_payload])?;

    let header = BtreePageHeader::parsing_handler(&page_type, to_parse_header)?;

    trace!("Parsed Btree page header: {header:?}");
    let header_length = header.length_bytes();

    let (_, to_parse_cell_pointer_array) =
      to_parse_header.split_at(header_length);

    let number_of_cells = header.number_of_cells().into_inner();
    let (raw_cell_pointer_array, to_parse_next) =
      to_parse_cell_pointer_array.split_at(usize::from(number_of_cells) * 2);

    let cell_pointer_array = BtreePageCellPointerArray::parsing_handler(
      &header,
      raw_cell_pointer_array,
    )?;
    trace!("Parsed Btree cell pointer array: {cell_pointer_array:?}");
    Ok(Self {
      page_type,
      header,
      cell_pointer_array,
    })
  }

  pub(crate) fn header(&self) -> &BtreePageHeader {
    &self.header
  }
}
