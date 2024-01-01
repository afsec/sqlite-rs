use super::common::{PagerNumberOfPages, PagerPageSize, PagerStorageMode};

#[derive(Debug)]
pub struct Pager<'a> {
  num_pages: PagerNumberOfPages,
  page_size: PagerPageSize,
  storage: PagerStorage<'a>,
}

#[derive(Debug)]
pub struct PagerStorage<'a> {
  mode: PagerStorageMode,
  engine: &'a [u8],
}
