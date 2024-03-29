use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Restore {
  #[arg(help = "Restore wallet from <MNEMONIC>")]
  mnemonic: Mnemonic,
  #[arg(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> when deriving wallet"
  )]
  pub(crate) passphrase: String,
}

impl Restore {
  pub(crate) fn run(self, wallet_name: String, options: Options) -> SubcommandResult {
    wallet::initialize(
      wallet_name,
      &options,
      self.mnemonic.to_seed(self.passphrase),
    )?;

    Ok(None)
  }
}
