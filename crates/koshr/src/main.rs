//! koshr: a git-compatible, self-hostable code store with signed, verified history.

use clap::{CommandFactory, Parser};

/// At 0.1.0 koshr has no subcommand: it answers `--help` and `--version`.
#[derive(Parser)]
#[command(
    name = "koshr",
    version,
    about = "koshr: a git-compatible, self-hostable code store with signed, verified history"
)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    // Bare `koshr` prints the usage and exits 0.
    let _ = Cli::command().print_help();
}
