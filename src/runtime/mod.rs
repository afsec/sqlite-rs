use crate::{
  header::SqliteHeader, pager::SqlitePager, result::SqliteResult,
  traits::ParseBytes,
};

#[derive(Debug)]
pub struct SqliteRuntime {
  pager: SqlitePager,
  header: SqliteHeader,
}

impl SqliteRuntime {
  pub fn start(mut pager: SqlitePager) -> SqliteResult<Self> {
    let header = SqliteHeader::parse_bytes(pager.first()?.get())?;
    Ok(Self { pager, header })
  }

  pub fn header(&self) -> &SqliteHeader {
    &self.header
  }
}
