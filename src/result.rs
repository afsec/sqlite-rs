use core::array::TryFromSliceError;
use core::fmt::Display;
#[cfg(feature = "std")]
use std::error::Error as StdError;
#[cfg(feature = "std")]
use std::io::Error as StdioError;

pub type SQLiteResult<T> = Result<T, SQLiteError>;

#[derive(Debug)]
pub enum SQLiteError {
  HeaderValidationError(&'static str),
  TryFromSliceError(TryFromSliceError),
  #[cfg(feature = "std")]
  StdioError(StdioError),
  Custom(&'static str),
  ParsingField(FieldParsingError),
  InvalidPayloadSize(InvalidPayloadSizeError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldParsingError {
  pub error: &'static str,
  pub ty: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidPayloadSizeError {
  pub error: &'static str,
  pub ty: &'static str,
}

impl Display for SQLiteError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    // TODO
    write!(f, "{:?}", self)
  }
}

impl From<TryFromSliceError> for SQLiteError {
  fn from(error: TryFromSliceError) -> Self {
    Self::TryFromSliceError(error)
  }
}

#[cfg(feature = "std")]
impl StdError for SQLiteError {}

#[cfg(feature = "std")]
impl From<StdioError> for SQLiteError {
  fn from(io_error: StdioError) -> Self {
    Self::StdioError(io_error)
  }
}
