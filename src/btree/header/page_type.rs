use crate::result::SQLiteError;
use crate::traits::Name;
use crate::{
  field_parsing_error, impl_name, result::SQLiteResult, traits::ParseBytes,
};

/// # BtreePageType (1 Byte)
///
///   - A value of `2` (0x02) means the page is an interior index b-tree page.
///   - A value of `5` (0x05) means the page is an interior table b-tree page.
///   - A value of `10` (0x0a) means the page is a leaf index b-tree page.
///   - A value of `13` (0x0d) means the page is a leaf table b-tree page.
///
/// Any other value for the b-tree page type is an error.
#[derive(Debug, PartialEq,Eq)]
pub enum BtreePageType {
  InteriorIndexPage,
  InteriorTablePage,
  LeafIndexPage,
  LeafTablePage,
}

impl_name! {BtreePageType}

impl TryFrom<u8> for BtreePageType {
  type Error = SQLiteError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    let outcome = match value {
      2 => Self::InteriorIndexPage,
      5 => Self::InteriorTablePage,
      10 => Self::LeafIndexPage,
      13 => Self::LeafTablePage,
      _ => return Err(field_parsing_error! {Self::NAME}),
    };
    Ok(outcome)
  }
}

impl ParseBytes for BtreePageType {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let value = bytes[0];
    value.try_into()
  }
}
