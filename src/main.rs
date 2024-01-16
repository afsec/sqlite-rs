mod sqlite_cli;

use self::sqlite_cli::{result::SqliteCliResult, SQliteCli};

fn main() -> SqliteCliResult<()> {
  let app = SQliteCli::parse()?;
  if app.cli().is_help() {
    app.usage();
    Ok(())
  } else {
    app.run()?;
    Ok(())
  }
}
