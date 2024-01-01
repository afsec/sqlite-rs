use super::common::{PagerNumberOfPages, PagerPageSize, PagerStorageMode};
use core::fmt::Debug;

use std::io::{Read, Write};

#[derive(Debug)]
pub struct Pager<T>
where
  T: StorageEngine + Debug,
{
  num_pages: PagerNumberOfPages,
  page_size: PagerPageSize,
  storage: PagerStorage<T>,
}

#[derive(Debug)]
pub struct PagerStorage<T>
where
  T: StorageEngine + Debug,
{
  mode: PagerStorageMode,
  engine: PagerStorageEngine<T>,
}

#[derive(Debug)]
pub struct PagerStorageEngine<T>(T)
where
  T: StorageEngine + Debug;

pub trait StorageEngine: Read + Write {}
