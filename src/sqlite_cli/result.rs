use sqlite_rs::result::SqliteError;
use std::convert::Infallible;
use std::io::Error as StdIoError;
use std::{fmt::Display, net::AddrParseError, num::ParseIntError};

pub(crate) type SqliteCliResult<T> = Result<T, SqliteCliError>;

#[derive(Debug)]
pub(crate) enum SqliteCliError {
  Infallible(Infallible),
  SqliteRsLib(SqliteError),
  Custom(String),
  StdIo(StdIoError),
  InvalidCLiArgs(String),
  AddrParseError(AddrParseError),
  ParseIntError(ParseIntError),
}

impl Display for SqliteCliError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
impl From<Infallible> for SqliteCliError {
  fn from(value: Infallible) -> Self {
    Self::Infallible(value)
  }
}
impl From<SqliteError> for SqliteCliError {
  fn from(value: SqliteError) -> Self {
    Self::SqliteRsLib(value)
  }
}
impl From<AddrParseError> for SqliteCliError {
  fn from(error: AddrParseError) -> Self {
    Self::AddrParseError(error)
  }
}

impl From<ParseIntError> for SqliteCliError {
  fn from(error: ParseIntError) -> Self {
    Self::ParseIntError(error)
  }
}

impl From<StdIoError> for SqliteCliError {
  fn from(value: StdIoError) -> Self {
    Self::StdIo(value)
  }
}
