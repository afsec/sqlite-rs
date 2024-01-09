use crate::traits::{Name, ParseBytes};
use crate::{field_parsing_error, impl_name, result::SqliteResult};
use core::fmt::Debug;
const SQLITE3_FILE_FORMAT_MAGIC_STRING: [u8; 16] = [
  0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
  0x20, 0x33, 0x00,
];

/// # Magic Header String (16 Bytes)
///
///  Every valid Sqlite database file begins with the following
/// 16 bytes (in hex): `53 51 4c 69 74 65 20 66 6f 72 6d 61 74 20 33 00`.
/// This byte sequence corresponds to the UTF-8 string `Sqlite format 3`
/// including the nul terminator character at the end.
#[derive()]
pub struct MagicHeaderString([u8; 16]);

impl_name! {MagicHeaderString}

impl Debug for MagicHeaderString {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple(Self::NAME).finish()
  }
}

impl ParseBytes for MagicHeaderString {
  const LENGTH_BYTES: usize = 16;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    for (idx, byte) in SQLITE3_FILE_FORMAT_MAGIC_STRING.iter().enumerate() {
      if bytes.get(idx) != Some(byte) {
        return Err(field_parsing_error! {Self::NAME.into()});
      }
    }

    Ok(Self(SQLITE3_FILE_FORMAT_MAGIC_STRING))
  }
}
