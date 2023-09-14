use crate::result::{InvalidPayloadSizeError, SQLiteError, SQLiteResult};

pub trait Name {
  const NAME: &'static str;
}

pub(super) trait ParseBytes
where
  Self: Sized + Name,
{
  const LENGTH_BYTES: usize;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self>;

  fn check_payload_size(bytes: &[u8]) -> SQLiteResult<()> {
    if bytes.len() < Self::LENGTH_BYTES {
      Err(SQLiteError::InvalidPayloadSize(InvalidPayloadSizeError {
        error: "Invalid input size",
        ty: Self::NAME,
      }))
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
