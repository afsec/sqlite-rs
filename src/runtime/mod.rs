use crate::result::SqliteResult;

#[derive(Debug)]
pub struct SqliteRuntime(());

impl SqliteRuntime {
  pub fn start() -> SqliteResult<Self> {
    Ok(Self(()))
  }
}
