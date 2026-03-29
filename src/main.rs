use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::time::Instant;

use clap::{Parser, ValueEnum};

use zed_convex::reporter::Reporter;
use zed_convex::reporter::cli::CliReporter;
use zed_convex::reporter::json::JsonReporter;

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Cli,
    Json,
}

#[derive(Parser)]
#[command(
    name = "zed-convex",
    version,
    about = "Diagnose your Convex backend in Zed editor"
)]
struct Cli {
    /// Path to the project root (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output format: cli, json
    #[arg(long, value_enum, default_value_t = OutputFormat::Cli)]
    format: OutputFormat,

    /// Only output the score (0-100)
    #[arg(long)]
    score: bool,

    /// Only analyze files changed vs this base branch
    #[arg(long)]
    diff: Option<String>,

    /// Show verbose output with file paths and line numbers
    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();
    let result = match zed_convex::engine::run(&cli.path, cli.verbose, cli.diff.as_deref()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };
    let elapsed = start.elapsed();

    if cli.score {
        let mut stdout = io::stdout();
        let output = zed_convex::reporter::score_only(&result.scoring);
        stdout
            .write_all(output.as_bytes())
            .expect("failed to write score output");
        stdout.flush().expect("failed to flush score output");
    } else {
        let output = match cli.format {
            OutputFormat::Json => {
                let reporter = JsonReporter;
                reporter.format(
                    &result.diagnostics,
                    &result.scoring,
                    &result.project_name,
                    cli.verbose,
                    result.files_scanned,
                    elapsed,
                )
            }
            OutputFormat::Cli => {
                let reporter = CliReporter;
                reporter.format(
                    &result.diagnostics,
                    &result.scoring,
                    &result.project_name,
                    cli.verbose,
                    result.files_scanned,
                    elapsed,
                )
            }
        };
        let mut stdout = io::stdout();
        stdout
            .write_all(output.as_bytes())
            .expect("failed to write report output");
        stdout.flush().expect("failed to flush report output");
    }

    if result.fail_below > 0 && result.scoring.value < result.fail_below {
        process::exit(1);
    }
}
