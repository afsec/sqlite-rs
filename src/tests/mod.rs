use crate::SqliteDatabase;
use std::dbg;

#[test]
fn it_works() {
  let db = SqliteDatabase::open(":memory:").unwrap();
  dbg!(db);
}
