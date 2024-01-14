mod help;
mod open;
mod sql;

use sqlite_rs::SqliteConnection;

use self::{help::ReplHelp, open::ReplOpen};

use super::{
  cli::Cli,
  result::{SqliteCliError, SqliteCliResult},
};

#[derive(Debug, Default)]
pub(crate) struct SqliteCliRepl {
  cli: Cli,
  conn: Option<SqliteConnection>,
}

impl SqliteCliRepl {
  pub(crate) fn start(cli: Cli) -> SqliteCliResult<()> {
    use std::io;
    use std::io::Write;
    println!(r#"Enter ".help" for usage hints."#);
    // println!("Connected to a transient in-memory database.");
    // println!(r#"Use ".open FILENAME" to reopen on a persistent database."#);
    let mut repl = Self {
      cli,
      ..Default::default()
    };
    let mut is_repl_running = true;

    while is_repl_running {
      let mut input = String::new();

      print!("sqlite-rs> ");

      io::stdout().flush()?;
      io::stdin().read_line(&mut input)?;

      let normalized_input = input.trim();
      if normalized_input.starts_with('.') {
        match normalized_input {
          ".quit" => is_repl_running = false,
          s => repl.internal_command(s)?,
        };
      } else {
        self::sql::run(normalized_input)?;
      }
    }
    Ok(())
  }
  fn internal_command(
    &mut self,
    normalized_input: impl AsRef<str>,
  ) -> SqliteCliResult<()> {
    let mut line = normalized_input.as_ref().split_ascii_whitespace();
    let command = line.next().ok_or(SqliteCliError::Custom(format!(
      "Impossible state in {} at line {}",
      file!(),
      line!()
    )))?;
    let maybe_arg1 = line.next().map(|s| s.to_owned());

    match command {
      ".help" => ReplHelp::run(maybe_arg1)?,
      ".open" => ReplOpen::run(maybe_arg1)?,
      s => println!("[{s}] it not implemented"),
    };
    Ok(())
  }
}
