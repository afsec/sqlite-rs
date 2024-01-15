use crate::sqlite_cli::result::SqliteCliResult;
pub(super) struct ReplOpen;
impl ReplOpen {
  pub(super) fn run(maybe_arg1: Option<String>) -> SqliteCliResult<()> {
    println!("[.open] it not implemented");
    // let conn = SqliteConnection::open(format!("sqlite://{file_path}"))?;
    Ok(())
  }
}
