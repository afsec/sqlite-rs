use crate::SqliteConnection;

#[test]
fn ok_on_new_inmemory_database() {
  let res = SqliteConnection::open_sample();
  dbg!(&res);
  assert!(res.is_ok());
  let mut conn = res.unwrap();
  dbg!(&conn);
  let page = conn.runtime_mut().pager_mut().first().unwrap();
  dbg!(page);
  let header = conn.runtime().header();
  assert_eq!(header.page_size(), conn.runtime().pager().page_size());
  assert_eq!(
    header.reserved_bytes_per_page(),
    conn.runtime().pager().reserved_bytes_per_page()
  );
  dbg!(&header);
}
