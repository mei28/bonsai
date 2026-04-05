use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;

use crate::cli::Cli;
use crate::error::Result;

/// Binary names to generate completions for
const BIN_NAMES: &[&str] = &["bonsai", "bn"];

pub fn exec(shell: Shell) -> Result<()> {
    for bin_name in BIN_NAMES {
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, *bin_name, &mut std::io::stdout());
    }
    Ok(())
}
