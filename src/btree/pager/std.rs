use super::{PagerNumberOfPages, PagerPageSize, PagerStorageMode};
use core::fmt::Debug;

#[cfg(feature = "std")]
use std::io::{Read, Write};

#[derive(Debug, PartialEq,Eq)]
pub struct Pager<T>
where
  T: StorageEngine + Debug,
{
  num_pages: PagerNumberOfPages,
  page_size: PagerPageSize,
  storage: PagerStorage<T>,
}

#[cfg(feature = "std")]
#[derive(Debug, PartialEq,Eq)]
pub struct PagerStorage<T>
where
  T: StorageEngine + Debug,
{
  mode: PagerStorageMode,
  engine: PagerStorageEngine<T>,
}
#[derive(Debug, PartialEq,Eq)]
pub struct PagerStorageEngine<T>(T)
where
  T: StorageEngine + Debug;

pub trait StorageEngine: Read + Write {}
