mod sqlite_cli;

use self::sqlite_cli::{result::SqliteCliResult, SQliteCli};

fn main() -> SqliteCliResult<()> {
  println!(
    "{} v{} - {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("SQLITERS_BUILT_AT")
  );

  let app = SQliteCli::parse()?;
  if app.cli().is_help() {
    app.usage();
    Ok(())
  } else {
    app.run()?;
    Ok(())
  }
}
