use super::traits::ParseBytes;
use crate::result::SQLiteResult;
use core::ops::Deref;

/// # Schema cookie (4 Bytes)
///
///  The schema cookie is a 4-byte big-endian integer at offset 40 that is
/// incremented whenever the database schema changes. A prepared statement is
/// compiled against a specific version of the database schema. When the
/// database schema changes, the statement must be reprepared. When a prepared
/// statement runs, it first checks the schema cookie to ensure the value is the
/// same as when the statement was prepared and if the schema cookie has
/// changed, the statement either automatically reprepares and reruns or it
/// aborts with an [SQLITE_SCHEMA](https://www.sqlite.org/rescode.html#schema)
/// error.
#[derive(Debug)]
pub struct SchemaCookie(u32);

impl Deref for SchemaCookie {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl ParseBytes for SchemaCookie {
  const NAME: &'static str = "SchemaCookie";
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SQLiteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let database_size = u32::from_be_bytes(buf);

    Ok(Self(database_size))
  }
}
