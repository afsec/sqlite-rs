use crate::result::{SQLiteError, SQLiteResult};
use alloc::format;

pub(super) trait ParseBytes<T>
where
  Self: Sized,
{
  fn struct_name() -> &'static str;
  fn bytes_length() -> usize;
  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self>;

  fn check_payload_size(bytes: &[u8]) -> SQLiteResult<()> {
    if bytes.len() != Self::bytes_length() {
      Err(SQLiteError::Custom(format!(
        "Invalid size for {}",
        Self::struct_name()
      )))
    } else {
      Ok(())
    }
  }
  fn parse_bytes(bytes: &[u8]) -> SQLiteResult<Self> {
    Self::check_payload_size(bytes)?;
    Self::parsing_handler(bytes)
  }
}

// TODO
pub(super) trait ValidateParsed<T>
where
  Self: Sized + ParseBytes<T>,
{
  fn validate_parsed(&self) -> SQLiteResult<()>;
}
