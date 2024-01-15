use super::traits::ArgName;

#[derive(Debug)]
pub(crate) struct CliHelp;

impl ArgName for CliHelp {
  fn arg_name() -> String {
    "--help".into()
  }
}
