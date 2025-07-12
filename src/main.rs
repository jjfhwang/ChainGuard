// src/main.rs
/*
 * Main executable for ChainGuard
 */

use clap::Parser;
use chainguard::{Result, run};

#[derive(Parser)]
#[command(version, about = "ChainGuard - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
