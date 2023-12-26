use crate::SqliteConnection;

#[test]
fn ok_on_new_inmemory_database() {
  let res = SqliteConnection::open_sample();

  assert!(res.is_ok());
  let conn = res.unwrap();
  dbg!(&conn);
  
  let header = conn.runtime().header();

  dbg!(&header);
}
