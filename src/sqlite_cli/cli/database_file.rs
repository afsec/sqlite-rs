use super::traits::ArgName;
use std::ops::Deref;

// TODO: Implement Path or PathBuf
#[derive(Debug)]
pub(crate) struct CliDatabaseFile(String);
impl Default for CliDatabaseFile {
  fn default() -> Self {
    Self("database.sqlite3".into())
  }
}

impl ArgName for CliDatabaseFile {
  fn arg_name() -> String {
    "--database-name".into()
  }
}

impl From<String> for CliDatabaseFile {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl Deref for CliDatabaseFile {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
