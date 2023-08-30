use super::traits::ParseBytes;
use crate::result::{SQLiteError, SQLiteResult};
use alloc::format;
use core::fmt::Debug;

/// Reserved for expansion. Must be zero. (20 Bytes)
#[derive(Default)]
pub struct ReservedForExpansion([u8; 20]);

impl Debug for ReservedForExpansion {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let output = format!("{:02x?}", self.0);
    f.debug_tuple("ReservedForExpansion")
      .field(&output)
      .finish()
  }
}

impl ParseBytes<&[u8]> for ReservedForExpansion {
  const NAME: &'static str = "ReservedForExpansion";

  const LENGTH_BYTES: usize = 20;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    for byte in bytes.iter() {
      if byte != &b'\0' {
        return Err(SQLiteError::Custom(format!(
          "Invalid payload for {}",
          Self::NAME
        )));
      }
    }
    Ok(Default::default())
  }
}
