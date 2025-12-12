use clap::{Args, Parser, Subcommand};
use devx_cmd::cmd;
use devx_pre_commit::locate_project_root;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor as _};
use semver::{Version, VersionReq};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub fn eval(self) -> anyhow::Result<()> {
        match Cli::parse().command {
            Command::InstallPreCommitHook => run_install_pre_commit_hook(),
            Command::InstallDependencies => run_install_dependencies(),
            Command::Format(format_opts) => run_format(format_opts),
            Command::Clippy => run_clippy(),
            Command::RegularTest => run_regular_tests(),
            Command::WasmTest(wasm_test_opts) => run_wasm_tests(wasm_test_opts),
            Command::Checks => run_checks(),
            Command::ServeBook => run_serve_book(),
            Command::BookGenerateExamples(v) => run_book_generate_examples(v),
        }
    }
}

#[derive(Subcommand)]
enum Command {
    InstallPreCommitHook,
    InstallDependencies,
    Format(FormatOpts),
    Clippy,
    RegularTest,
    WasmTest(WasmTestOpts),
    Checks,
    ServeBook,
    #[command(hide = true)]
    BookGenerateExamples(BookGenerateExamples),
}

#[derive(Args)]
struct InstallDependenciesOpts {
    #[command(subcommand)]
    mode: Option<InstallDependenciesMode>,
}

#[derive(Copy, Clone, Subcommand)]
enum InstallDependenciesMode {
    /// Install dependencies from pre-compiled binaries
    CI,
    /// Install dependencies through `cargo install`
    Development,
}

#[derive(Args)]
struct FormatOpts {
    /// Whether to check only or not
    #[arg(short, long)]
    check_only: bool,
}

#[derive(Args)]
struct WasmTestOpts {
    #[command(subcommand)]
    mode: Option<WasmTestMode>,
}

#[derive(Copy, Clone, Subcommand)]
enum WasmTestMode {
    /// Test with headless Chrome and Firefox
    HeadlessChromeAndFirefox,
    /// Test with headless Chrome
    HeadlessChrome,
    /// Test with headless Firefox,
    HeadlessFirefox,
    /// Test with non-headless Chrome
    Chrome,
    /// Test with non-headless Firefox,
    Firefox,
}

#[derive(Args)]
struct BookGenerateExamples {
    #[command(subcommand)]
    supports: Option<BookGenerateExamplesSupports>,
}

#[derive(Subcommand)]
enum BookGenerateExamplesSupports {
    Supports {
        #[arg(required = true)]
        renderer: String,
    },
}

fn run_install_pre_commit_hook() -> anyhow::Result<()> {
    devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;
    Ok(())
}

fn run_install_dependencies() -> anyhow::Result<()> {
    let root = locate_project_root()?;

    cmd!("cargo", "install", "wasm-pack", "--version", "^0.13")
        .current_dir(&root)
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;
    cmd!("cargo", "install", "trunk", "--version", "^0.21")
        .current_dir(&root)
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;
    cmd!("cargo", "install", "mdbook", "--version", "^0.4")
        .current_dir(&root)
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;

    Ok(())
}

fn run_format(opts: FormatOpts) -> anyhow::Result<()> {
    let root = locate_project_root()?;

    if opts.check_only {
        cmd!("cargo", "fmt", "--check")
    } else {
        cmd!("cargo", "fmt")
    }
    .current_dir(&root)
    .log_cmd(log::Level::Trace)
    .log_err(log::Level::Trace)
    .run()?;

    Ok(())
}

fn run_clippy() -> anyhow::Result<()> {
    let root = locate_project_root()?;

    cmd!("cargo", "clippy", "--all-targets")
        .current_dir(&root)
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;

    Ok(())
}

fn run_regular_tests() -> anyhow::Result<()> {
    let root = locate_project_root()?;

    cmd!("cargo", "test")
        .current_dir(&root)
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;

    Ok(())
}

fn run_wasm_tests(opts: WasmTestOpts) -> anyhow::Result<()> {
    let root = locate_project_root()?;

    match opts.mode.unwrap_or(WasmTestMode::HeadlessChromeAndFirefox) {
        WasmTestMode::HeadlessChromeAndFirefox => {
            cmd!("wasm-pack", "test", "--headless", "--chrome", "--firefox")
        }
        WasmTestMode::HeadlessChrome => {
            cmd!("wasm-pack", "test", "--headless", "--chrome")
        }
        WasmTestMode::HeadlessFirefox => {
            cmd!("wasm-pack", "test", "--headless", "--firefox")
        }
        WasmTestMode::Chrome => {
            cmd!("wasm-pack", "test", "--chrome")
        }
        WasmTestMode::Firefox => {
            cmd!("wasm-pack", "test", "--firefox")
        }
    }
    .current_dir(&root)
    .log_cmd(log::Level::Trace)
    .log_err(log::Level::Trace)
    .run()?;

    Ok(())
}

pub fn run_checks() -> anyhow::Result<()> {
    run_format(FormatOpts { check_only: true })?;
    run_clippy()?;
    run_regular_tests()?;
    run_wasm_tests(WasmTestOpts {
        mode: Some(WasmTestMode::HeadlessChromeAndFirefox),
    })?;

    Ok(())
}

fn run_serve_book() -> anyhow::Result<()> {
    let root = locate_project_root()?;

    cmd!("mdbook", "serve")
        .current_dir(root.join("doc"))
        .log_cmd(log::Level::Trace)
        .log_err(log::Level::Trace)
        .run()?;

    Ok(())
}

fn run_book_generate_examples(cmd: BookGenerateExamples) -> anyhow::Result<()> {
    let mut examples_dir = locate_project_root()?;
    examples_dir.push("examples");
    let pre = crate::book::ExamplesPreprocessor { examples_dir };

    if let Some(supports) = cmd.supports {
        let renderer = match supports {
            BookGenerateExamplesSupports::Supports { renderer } => renderer,
        };
        if pre.supports_renderer(&renderer) {
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "").into())
        }
    } else {
        let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;

        let book_version = Version::parse(&ctx.mdbook_version)?;
        let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

        if !version_req.matches(&book_version) {
            eprintln!(
                "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
                pre.name(),
                mdbook::MDBOOK_VERSION,
                ctx.mdbook_version
            );
        }

        let processed_book = pre.run(&ctx, book)?;
        serde_json::to_writer(std::io::stdout(), &processed_book)?;

        Ok(())
    }
}
