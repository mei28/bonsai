pub mod app;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod git;
pub mod hooks;

use clap::Parser;

pub fn run() -> error::Result<()> {
    let cli = cli::Cli::parse();

    let no_color = cli.no_color || std::env::var("NO_COLOR").is_ok();

    if no_color {
        colored::control::set_override(false);
    }

    app::dispatch(cli.command, cli.dry_run, cli.verbose, no_color)
}
