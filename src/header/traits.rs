use crate::result::{SQLiteError, SQLiteResult};

pub(super) trait ParseBytes
where
  Self: Sized,
{
  /// Workaround for unstable:
  /// `std::any::type_name()`
  const NAME: &'static str;

  const LENGTH_BYTES: usize;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self>;

  fn check_payload_size(bytes: &[u8]) -> SQLiteResult<()> {
    if bytes.len() < Self::LENGTH_BYTES {
      Err(SQLiteError::Custom(
        "Invalid input size on ParseBytes trait",
      ))
    } else {
      Ok(())
    }
  }
  fn parse_bytes(bytes: &[u8]) -> SQLiteResult<Self> {
    Self::check_payload_size(bytes)?;
    Self::parsing_handler(bytes)
  }
}

pub(super) trait ValidateParsed
where
  Self: Sized + ParseBytes,
{
  fn validate_parsed(&self) -> SQLiteResult<()>;
}
