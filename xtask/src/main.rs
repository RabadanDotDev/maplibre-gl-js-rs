use anyhow::Ok;
use clap::Parser as _;
use xtask::{Cli, run_if_precommit};

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    if run_if_precommit()? {
        Ok(())
    } else {
        Cli::parse().eval()
    }
}
