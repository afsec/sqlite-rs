use core::array::TryFromSliceError;
use core::fmt::Display;
use std::error::Error as StdError;
use std::io::Error as StdioError;

pub type SqliteResult<T> = Result<T, SqliteError>;

#[derive(Debug)]
pub enum SqliteError {
  EmptyDb,
  HeaderValidationError(String),
  TryFromSliceError(TryFromSliceError),
  StdioError(StdioError),
  Custom(String),
  ParsingField(FieldParsingError),
  InvalidPayloadSize(InvalidPayloadSizeError),
}

#[derive(Debug)]
pub struct FieldParsingError {
  pub error: String,
  pub ty: String,
}

#[derive(Debug)]
pub struct InvalidPayloadSizeError {
  pub error: String,
  pub ty: String,
}

impl Display for SqliteError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    // TODO
    write!(f, "{:?}", self)
  }
}

impl From<TryFromSliceError> for SqliteError {
  fn from(error: TryFromSliceError) -> Self {
    Self::TryFromSliceError(error)
  }
}

impl StdError for SqliteError {}

impl From<StdioError> for SqliteError {
  fn from(io_error: StdioError) -> Self {
    Self::StdioError(io_error)
  }
}
