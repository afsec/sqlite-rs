use crate::{impl_name, result::SQLiteResult, traits::ParseBytes};

#[derive(Debug, PartialEq,Eq)]
pub struct BtreePageNumberOfCells(u16);

impl_name! {BtreePageNumberOfCells}

impl ParseBytes for BtreePageNumberOfCells {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let value = u16::from_be_bytes([bytes[0], bytes[1]]);
    Ok(Self(value))
  }
}
