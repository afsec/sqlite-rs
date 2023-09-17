#[derive(Debug, PartialEq, Eq)]
pub enum PagerStorageMode {
  ReadAndWrite,
  ReadOnly,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PagerNumberOfPages;

#[derive(Debug, PartialEq, Eq)]
pub struct PagerPageSize;
