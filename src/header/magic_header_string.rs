use super::ParseBytes;
use anyhow::bail;
use std::fmt::Debug;

const SQLITE3_FILE_FORMAT_MAGIC_STRING: [u8; 16] = [
  0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
  0x20, 0x33, 0x00,
];

/// # Magic Header String (16 Bytes)
///  Every valid SQLite database file begins with the following
/// 16 bytes (in hex): `53 51 4c 69 74 65 20 66 6f 72 6d 61 74 20 33 00`.
/// This byte sequence corresponds to the UTF-8 string `SQLite format 3`
/// including the nul terminator character at the end.
pub struct MagicHeaderString([u8; 16]);

impl Debug for MagicHeaderString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let output = format!("{:02x?}", self.0);
    f.debug_tuple("MagicHeaderString").field(&output).finish()
  }
}

impl<'a> ParseBytes<&[u8]> for MagicHeaderString {
  fn struct_name() -> &'static str {
    "MagicHeaderString"
  }

  fn bytes_length() -> usize {
    16
  }

  fn parsing_handler(input: &[u8]) -> crate::result::SQLiteResult<Self> {
    let bytes = input;
    Self::check_payload_size(bytes)?;

    for (idx, byte) in SQLITE3_FILE_FORMAT_MAGIC_STRING.iter().enumerate() {
      if bytes.get(idx) != Some(byte) {
        bail!("Invalid payload for {}", Self::struct_name());
      }
    }

    Ok(Self(SQLITE3_FILE_FORMAT_MAGIC_STRING))
  }
}
