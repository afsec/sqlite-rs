use crate::result::{BtreeError, BtreeErrorParsing, SqliteError, SqliteResult};
use crate::traits::ParseBytes;

#[derive(Debug)]
pub(crate) enum BtreePageType {
  /// A value of 2 (0x02) means the page is an interior index b-tree page.
  InteriorIndex,
  /// A value of 5 (0x05) means the page is an interior table b-tree page.
  InteriorTable,
  /// A value of 10 (0x0a) means the page is a leaf index b-tree page.
  LeafIndex,
  /// A value of 13 (0x0d) means the page is a leaf table b-tree page.
  LeafTable,
}

impl_name! {BtreePageType}

impl ParseBytes for BtreePageType {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    use crate::traits::Name;
    let payload = *bytes
      .first()
      .ok_or(field_parsing_error! {Self::NAME.into()})?;
    let page_type = match payload {
      2 => Self::InteriorIndex,
      5 => Self::InteriorTable,
      10 => Self::LeafIndex,
      13 => Self::LeafTable,
      _ => {
        return Err(SqliteError::Btree(BtreeError::Parsing(
          BtreeErrorParsing::HeaderInvalidPayloadForPageType,
        )))
      }
    };
    Ok(page_type)
  }
}
