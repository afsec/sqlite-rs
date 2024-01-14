use crate::sqlite_cli::result::SqliteCliResult;
pub(super) struct ReplHelp;

impl ReplHelp {
  pub(super) fn run(maybe_arg1: Option<String>) -> SqliteCliResult<()> {
    match maybe_arg1 {
      None => Self::print(),
      Some(command) => Self::help(command)?,
    };

    Ok(())
  }
  fn print() {
    let commands = [
      ".dbinfo ?DB?             Show status information about the database",
      ".open ?OPTIONS? ?FILE?   Close existing database and reopen FILE",
      ".quit                    Exit this program",
    ];
    commands.iter().for_each(|s| println!("{s}"));
  }
  fn help(command: String) -> SqliteCliResult<()> {
    todo!();
    Ok(())
  }
}
