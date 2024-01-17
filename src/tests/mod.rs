use crate::{debug, trace, SqliteConnection};

#[test]
fn ok_on_new_inmemory_database() {
  #[cfg(feature = "log")]
  let res = SqliteConnection::open(":memory:");
  debug!("{res:?}");
  assert!(res.is_ok());
  let mut conn = res.unwrap();
  debug!("{conn:?}");
  let page = conn.runtime_mut().pager_mut().first().unwrap();
  trace!("{page:?}");
  let header = conn.runtime().header();
  assert_eq!(header.page_size(), conn.runtime().pager().page_size());
  assert_eq!(
    header.reserved_bytes_per_page(),
    conn.runtime().pager().reserved_bytes_per_page()
  );
  debug!("{header:?}");
}
