use crate::result::SqliteResult;

pub mod pager;

#[derive(Debug)]
pub struct SqliteRuntime(());

impl SqliteRuntime {
  pub fn start() -> SqliteResult<Self> {
    Ok(Self(()))
  }
}
