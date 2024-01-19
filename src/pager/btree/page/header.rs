use crate::pager::btree::header_fields::{
  FirstFreeBlock, FragmentedFreeBytes, NumberOfCells, RightMostPointer,
  StartofCellContentArea,
};
use crate::pager::btree::page::{
  interior::BtreePageHeaderInterior, leaf::BtreePageHeaderLeaf,
};
use crate::pager::btree::page_type::BtreePageType;
use crate::result::SqliteResult;
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
