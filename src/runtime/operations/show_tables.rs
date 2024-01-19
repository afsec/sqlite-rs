use crate::{
  pager::{btree::page::BtreePage, SqlitePager},
  result::SqliteResult,
};

pub(crate) struct ShowTables;

impl ShowTables {
  pub(crate) fn run(pager: &mut SqlitePager) -> SqliteResult<Vec<String>> {
    let sqlite_master = Self::sqlite_master(pager)?;
    let second_page = pager.read(2)?;
    dbg!(second_page);
    todo!("Show tables not implemented");
    Ok(vec![])
  }
  fn sqlite_master(pager: &mut SqlitePager) -> SqliteResult<SqliteMaster> {
    use crate::runtime::SqliteHeader;
    let (page_size, mut bytes) = pager.first()?.take();
    let (_, to_parse) = bytes.split_at(SqliteHeader::LENGTH_BYTES);
    let btree_page = BtreePage::parsing_handler(to_parse)?;

    todo!();
    Ok(SqliteMaster)
  }
}

pub(crate) struct SqliteMaster;
