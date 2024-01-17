mod sqlite_cli;

use self::sqlite_cli::SQliteCli;
use sqlite_cli::result::SqliteCliResult;
use std::process::ExitCode;

fn main() -> ExitCode {
  match smain() {
    Ok(_) => ExitCode::SUCCESS,
    Err(_) => ExitCode::FAILURE,
  }
}

fn smain() -> SqliteCliResult<()> {
  let app = SQliteCli::parse()?;
  if app.cli().is_help() {
    app.usage();
    Ok(())
  } else {
    app.run()?;
    Ok(())
  }
}
