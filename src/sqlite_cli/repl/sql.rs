use sqlite_rs::sql::keywords::SQLITE_KEYWORDS;

use crate::sqlite_cli::result::{
  SqliteCliError, SqliteCliResult, SqliteReplError,
};

pub(super) fn run(normalized_input: impl AsRef<str>) -> SqliteCliResult<()> {
  let mut split = normalized_input.as_ref().split_ascii_whitespace();
  let keyword = split
    .next()
    .ok_or(SqliteCliError::Repl(SqliteReplError::NoKeyword))?;
  if SQLITE_KEYWORDS.contains(&keyword) {
    println!("SQL queries still not implemented.");
  } else {
    println!("Invalid input: [{}].", normalized_input.as_ref());
  }

  Ok(())
}
