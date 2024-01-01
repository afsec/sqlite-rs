#[derive(Debug)]
pub enum PagerStorageMode {
  ReadAndWrite,
  ReadOnly,
}

#[derive(Debug)]
pub struct PagerNumberOfPages;

#[derive(Debug)]
pub struct PagerPageSize;
