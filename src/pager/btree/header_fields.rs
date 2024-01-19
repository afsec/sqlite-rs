use std::num::NonZeroU16;

use crate::{result::SqliteResult, traits::ParseBytes};

#[derive(Debug)]
pub(crate) struct FirstFreeBlock(Option<u16>);
impl_name! {FirstFreeBlock}

impl ParseBytes for FirstFreeBlock {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let parsed = NonZeroU16::new(u16::from_be_bytes([bytes[0], bytes[1]]))
      .map(|not_zero| not_zero.get());
    Ok(Self(parsed))
  }
}

#[derive(Debug)]
pub(crate) struct NumberOfCells(u16);
impl NumberOfCells {
  pub(crate) fn into_inner(&self) -> u16 {
    self.0
  }
}
impl_name! {NumberOfCells}

impl ParseBytes for NumberOfCells {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let parsed = u16::from_be_bytes([bytes[0], bytes[1]]);
    Ok(Self(parsed))
  }
}

#[derive(Debug)]
// TODO: Implemement like were in PageSize (two bytes, but 0 => 65536)
pub(crate) struct StartofCellContentArea(u32);
impl_name! {StartofCellContentArea}
impl ParseBytes for StartofCellContentArea {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let parsed = match NonZeroU16::new(u16::from_be_bytes([bytes[0], bytes[1]]))
    {
      Some(non_zero) => non_zero.get().into(),
      None => 65536,
    };
    Ok(Self(parsed))
  }
}

#[derive(Debug)]
pub(crate) struct FragmentedFreeBytes(u8);
impl_name! {FragmentedFreeBytes}
impl ParseBytes for FragmentedFreeBytes {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let parsed = u8::from(bytes[0]);
    Ok(Self(parsed))
  }
}

// * For interior only
#[derive(Debug)]
pub(crate) struct RightMostPointer(u32);
impl_name! {RightMostPointer}
impl ParseBytes for RightMostPointer {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    todo!()
  }
}
