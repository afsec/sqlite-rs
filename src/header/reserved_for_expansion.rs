use super::traits::ParseBytes;
use crate::result::{SQLiteError, SQLiteResult};

use core::fmt::Debug;

/// Reserved for expansion. Must be zero. (20 Bytes)
#[derive(Default)]
pub struct ReservedForExpansion([u8; 20]);

impl Debug for ReservedForExpansion {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple(Self::NAME).finish()
  }
}

impl ParseBytes for ReservedForExpansion {
  const NAME: &'static str = "ReservedForExpansion";

  const LENGTH_BYTES: usize = 20;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    for byte in bytes.iter() {
      if *byte != b'\0' {
        return Err(SQLiteError::Custom(
          "Invalid payload for ReservedForExpansion",
        ));
      }
    }
    Ok(Default::default())
  }
}
