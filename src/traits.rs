use crate::result::{InvalidPayloadSizeError, SqliteError, SqliteResult};
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

pub trait Name {
  const NAME: &'static str;
}

pub(super) trait ParseBytes
where
  Self: Sized + Name,
{
  const LENGTH_BYTES: usize;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self>;

  fn check_payload_size(bytes: &[u8]) -> SqliteResult<()> {
    if bytes.len() < Self::LENGTH_BYTES {
      Err(SqliteError::InvalidPayloadSize(InvalidPayloadSizeError {
        error: "Invalid input size".into(),
        ty: Self::NAME.into(),
      }))
    } else {
      Ok(())
    }
  }
  fn parse_bytes(bytes: &[u8]) -> SqliteResult<Self> {
    Self::check_payload_size(bytes)?;
    Self::parsing_handler(bytes)
  }
}

pub(crate) trait ValidateParsed
where
  Self: Sized + ParseBytes,
{
  fn validate_parsed(&self) -> SqliteResult<()>;
}

// pub(crate) trait SqliteRawIo: Read + Send + Sync + Write {}
pub(crate) trait SqliteRawIo: Read + Send + Sync + Seek {}
impl SqliteRawIo for Cursor<Vec<u8>> {}
impl SqliteRawIo for File {}
