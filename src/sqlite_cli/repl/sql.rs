use crate::sqlite_cli::result::SqliteCliResult;

pub(super) fn run(normalized_input: impl AsRef<str>) -> SqliteCliResult<()> {
  println!("SQL queries is not implemented");
  Ok(())
}
