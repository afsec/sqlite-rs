mod database_file;
mod help;
mod traits;

use self::database_file::CliDatabaseFile;
use self::help::CliHelp;
use self::traits::ArgName;
use super::result::SqliteCliError;
use std::collections::HashMap;
use std::{env::Args, ops::Not};

#[derive(Debug, Default)]
pub(crate) struct Cli {
  is_help: bool,
  database_file: Option<CliDatabaseFile>,
}

impl Cli {
  pub(crate) fn is_help(&self) -> bool {
    self.is_help
  }

  pub(crate) fn database_file(&self) -> Option<&CliDatabaseFile> {
    self.database_file.as_ref()
  }
}

const CLI_NUM_FIELDS: usize = 2;

impl From<CliArgs> for Cli {
  fn from(mut value: CliArgs) -> Self {
    let is_help = value.0.contains_key(&CliHelp::arg_name());
    let database_file = value
      .0
      .remove(&CliDatabaseFile::arg_name())
      .map(|v| v.into());
    Self {
      is_help,
      database_file,
    }
  }
}
impl TryFrom<Args> for Cli {
  type Error = SqliteCliError;

  fn try_from(args: Args) -> Result<Self, Self::Error> {
    let args_len = args.len();

    if (args_len > 0 && args_len < (CLI_NUM_FIELDS + 2)).not() {
      return Err(SqliteCliError::InvalidCLiArgs(format!(
        "Invalid args length: {args_len}"
      )));
    }

    let mut cli_args = CliArgs::new();

    for arg_to_parse in args.skip(1) {
      let mut arg = arg_to_parse.split('=');
      let k = arg.next();
      let v = arg.collect::<Vec<_>>().join("=");
      match (k, v) {
        (Some("--database-file"), value) => {
          cli_args.add((CliDatabaseFile::arg_name(), value))
        }
        (Some("--help"), _) => {
          cli_args.add((CliHelp::arg_name(), Default::default()))
        }
        _ => {
          println!(
            "{}: Error: unknown option: {}",
            env!("CARGO_PKG_NAME"),
            arg_to_parse
          );
          println!("Use --help for a list of options.");
          return Err(SqliteCliError::InvalidCLiArgs(arg_to_parse));
        }
      }
    }
    Ok(cli_args.into())
  }
}

// CliArg input sample:
//                                Mode is optional (`?mode=...`)
//                                              |
//                                          /-------\
//  --database-file="./some-database.sqlite3?mode=rwc"
//  ^^             ^^                                ^
//  ||             ||                                |
//  ++-------------++--------------------------------+-- Required characters

#[derive(Debug)]
pub(crate) struct CliArgs(HashMap<String, String>);

impl CliArgs {
  pub(crate) fn new() -> Self {
    Self(HashMap::new())
  }
  pub(crate) fn add(&mut self, (k, v): (String, String)) {
    self.0.insert(k, v);
  }
}
