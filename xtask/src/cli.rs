use clap::{Parser, Subcommand};
use devx_pre_commit::locate_project_root;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn eval(self) -> anyhow::Result<()> {
        match Cli::parse().command {
            Command::InstallPreCommitHook => run_install_pre_commit_hook(),
        }
    }
}

#[derive(Subcommand)]
enum Command {
    InstallPreCommitHook,
}

fn run_install_pre_commit_hook() -> anyhow::Result<()> {
    devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;
    Ok(())
}
