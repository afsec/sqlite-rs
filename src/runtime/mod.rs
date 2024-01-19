mod internal_tables;
mod operations;
mod schema;

use crate::{
  header::SqliteHeader, pager::SqlitePager, result::SqliteResult,
  traits::ParseBytes,
};

use self::operations::show_tables::ShowTables;
pub use self::schema::SqliteSchema;

#[derive(Debug)]
pub struct SqliteRuntime {
  pager: SqlitePager,
  header: SqliteHeader,
}

impl SqliteRuntime {
  pub fn start(mut pager: SqlitePager) -> SqliteResult<Self> {
    let header = if pager.io_mut().is_empty()? {
      SqliteHeader::default()
    } else {
      let first_page = pager.first()?;
      let (file_header, _) =
        first_page.raw_data().split_at(SqliteHeader::LENGTH_BYTES);
      SqliteHeader::parse_bytes(file_header)?
    };

    Ok(Self { pager, header })
  }

  pub fn header(&self) -> &SqliteHeader {
    &self.header
  }

  // pub fn tables(&mut self) -> SqliteResult<Vec<SqliteSchema>> {
  pub fn tables(&mut self) -> SqliteResult<Vec<String>> {
    ShowTables::run(&mut self.pager)
  }

  pub fn pager(&self) -> &SqlitePager {
    &self.pager
  }

  pub fn pager_mut(&mut self) -> &mut SqlitePager {
    &mut self.pager
  }
}
