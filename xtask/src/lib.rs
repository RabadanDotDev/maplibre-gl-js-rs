use devx_pre_commit::{PreCommitContext, locate_project_root};

mod cli;

pub use cli::Cli;

pub fn run_if_precommit() -> anyhow::Result<bool> {
    if let Some(true) = std::env::args().next().map(|it| it.contains("pre-commit")) {
        println!("Running pre-commit hook...");

        // Format staged files
        let ctx = PreCommitContext::from_git_diff(locate_project_root()?)?;
        ctx.rustfmt()?;
        ctx.stage_new_changes()?;

        // Run checks
        cli::run_checks()?;

        anyhow::Ok(true)
    } else {
        anyhow::Ok(false)
    }
}
