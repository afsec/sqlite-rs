use super::traits::PrintHelp;
use crate::sqlite_cli::result::SqliteCliResult;
use sqlite_rs::SqliteConnection;

pub(super) struct ReplOpen;
impl ReplOpen {
  pub(super) fn run(
    maybe_arg1: Option<String>,
  ) -> SqliteCliResult<SqliteConnection> {
    let conn = match maybe_arg1 {
      Some(file_path) => {
        SqliteConnection::open(format!("sqlite://{}", file_path.as_str()))?
      }
      None => SqliteConnection::open(":memory:")?,
    };
    Ok(conn)
  }
}
impl PrintHelp for ReplOpen {
  fn help() -> SqliteCliResult<()> {
    let help = [
      ".open ?OPTIONS? ?FILE?   Close existing database and reopen FILE",
      // TODO
      // "Options:",
      // "--new           Initialize FILE to an empty database",
      // "--nofollow      Do not follow symbolic links",
      // "--readonly      Open FILE readonly",
    ];
    help.iter().for_each(|line| println!("{line}"));
    Ok(())
  }
}
