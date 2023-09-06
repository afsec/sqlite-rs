use crate::{result::SQLiteResult, SQLiteDatabase};

mod flights_db;
mod helpers;

#[test]
fn it_works() -> SQLiteResult<()> {
  use self::flights_db::FLIGHTS_INITIAL_DB;

  let sqlite_database = SQLiteDatabase::try_from(&FLIGHTS_INITIAL_DB[..])?;

  #[cfg(feature = "std")]
  std::dbg!(sqlite_database);

  Ok(())
}
