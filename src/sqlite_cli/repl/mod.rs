mod dbinfo;
mod help;
mod open;
mod sql;
mod traits;

use self::{dbinfo::ReplDbInfo, help::ReplHelp, open::ReplOpen};
use super::{
  cli::Cli,
  result::{SqliteCliError, SqliteCliResult},
};
use sqlite_rs::{io::SqliteIoMode, SqliteConnection};

#[derive(Debug)]
pub(crate) struct SqliteCliRepl {
  is_tty: bool,
  cli: Cli,
  conn: SqliteConnection,
}

impl SqliteCliRepl {
  pub(crate) fn start(cli: Cli) -> SqliteCliResult<()> {
    let mut repl = Self::new(cli)?;
    if repl.is_tty {
      repl.main_loop()
    } else {
      repl.run_from_pipe()
    }
  }
  fn new(cli: Cli) -> SqliteCliResult<Self> {
    use std::io::stdin;
    use std::io::IsTerminal;

    let conn = match cli.database_file() {
      Some(file_path) => {
        SqliteConnection::open(format!("sqlite://{}", file_path.as_str()))?
      }
      None => SqliteConnection::open(":memory:")?,
    };

    let is_tty = stdin().is_terminal();

    Ok(Self { cli, conn, is_tty })
  }
  fn run_from_pipe(&mut self) -> SqliteCliResult<()> {
    use std::io;
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let normalized_input = input.trim();
    if normalized_input.starts_with('.') {
      match normalized_input {
        ".quit" => (),
        s => self.internal_command(s)?,
      };
    } else {
      self::sql::run(normalized_input)?;
    }
    Ok(())
  }
  fn main_loop(&mut self) -> SqliteCliResult<()> {
    use std::io;
    use std::io::Write;
    println!(
      "{} v{} - {}",
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_VERSION"),
      env!("SQLITERS_BUILT_AT")
    );
    println!(r#"Enter ".help" for usage hints."#);
    if *self.conn.runtime().pager().io().mode() == SqliteIoMode::InMemory {
      println!("Connected to a transient in-memory database.");
      println!(r#"Use ".open FILENAME" to reopen on a persistent database."#);
    }
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
          s => self.internal_command(s)?,
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
      ".dbinfo" => ReplDbInfo::run(&mut self.conn)?,
      ".open" => {
        let new_conn = ReplOpen::run(maybe_arg1)?;
        self.conn = new_conn;
      }

      s => println!(
        r#"Error: unknown command or invalid arguments:  "{s}". Enter ".help" for help"#
      ),
    };
    Ok(())
  }
}
