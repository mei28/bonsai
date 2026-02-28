use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell;

use crate::cli::Cli;
use crate::error::Result;

pub fn exec(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "bonsai", &mut std::io::stdout());
    Ok(())
}
