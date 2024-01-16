pub(crate) mod cli;
pub(crate) mod repl;
pub(crate) mod result;

use crate::sqlite_cli::repl::SqliteCliRepl;

use self::{cli::Cli, result::SqliteCliResult};

#[derive(Debug, Default)]
pub(crate) struct SQliteCli {
  cli: Cli,
}

impl SQliteCli {
  pub(crate) fn usage(&self) {
    eprintln!(
      "Usage:{pkg_name} [OPTION]",
      pkg_name = env!("CARGO_PKG_NAME")
    );
    eprintln!("Options:");
    eprintln!("    --help                        Display this message");
    eprintln!("    --database-file=<FILENAME>    Database filename to open");
  }
  pub(crate) fn parse() -> SqliteCliResult<Self> {
    let cli = Cli::try_from(std::env::args())?;
    Ok(Self { cli })
  }

  pub(crate) fn run(self) -> SqliteCliResult<()> {
    SqliteCliRepl::start(self.cli)?;
    Ok(())
  }

  pub(crate) fn cli(&self) -> &Cli {
    &self.cli
  }
}
