#[cfg(feature = "alloc")]
use alloc::string::String;
use core::array::TryFromSliceError;
#[cfg(feature = "std")]
use std::io::Error as StdioError;

pub type SQLiteResult<T> = Result<T, SQLiteError>;

#[derive(Debug)]
pub enum SQLiteError {
  TryFromSliceError(TryFromSliceError),
  #[cfg(feature = "std")]
  StdioError(StdioError),
  #[cfg(feature = "alloc")]
  Custom(String),
}

impl SQLiteError {
  pub fn msg(msg: &str) -> Self {
    Self::Custom(msg.into())
  }
}

#[cfg(feature = "std")]
impl From<StdioError> for SQLiteError {
  fn from(io_error: StdioError) -> Self {
    Self::StdioError(io_error)
  }
}

#[cfg(feature = "alloc")]
impl From<String> for SQLiteError {
  fn from(s: String) -> Self {
    Self::Custom(s)
  }
}

impl From<TryFromSliceError> for SQLiteError {
  fn from(error: TryFromSliceError) -> Self {
    Self::TryFromSliceError(error)
  }
}
