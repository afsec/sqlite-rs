use super::traits::{Name, ParseBytes};
use crate::{
  field_parsing_error, impl_name,
  result::{SQLiteError, SQLiteResult},
};

use core::fmt::Debug;

/// Reserved for expansion. Must be zero. (20 Bytes)
#[derive(Default)]
pub struct ReservedForExpansion([u8; 20]);

impl Debug for ReservedForExpansion {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple(Self::NAME).finish()
  }
}

impl_name! {ReservedForExpansion}

impl ParseBytes for ReservedForExpansion {
  const LENGTH_BYTES: usize = 20;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    for byte in bytes.iter() {
      if *byte != b'\0' {
        return Err(field_parsing_error! {Self::NAME});
      }
    }
    Ok(Default::default())
  }
}
