use crate::sqlite_cli::result::SqliteCliResult;

pub(super) trait PrintHelp {
  fn help() -> SqliteCliResult<()>;
}
