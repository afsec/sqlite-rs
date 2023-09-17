#[cfg(not(feature = "std"))]
pub mod no_std;

#[cfg(feature = "std")]
pub mod std;

#[derive(Debug, PartialEq,Eq)]
pub enum PagerStorageMode {
  ReadAndWrite,
  ReadOnly,
}

#[derive(Debug, PartialEq,Eq)]
pub struct PagerNumberOfPages;

#[derive(Debug, PartialEq,Eq)]
pub struct PagerPageSize;
