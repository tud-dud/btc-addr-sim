use clap::Parser;
use log::{info, LevelFilter};
use std::path::PathBuf;
use types::Addrman;

mod addrman;
mod types;

#[derive(clap::Parser)]
#[command(version, about)]
/// Listen for TCP connections.
struct Cli {
    #[arg(long = "log", short = 'l', default_value = "info")]
    log_level: LevelFilter,
    /// Path to JSON file containing the addrman dump
    #[arg(long = "addrman", short = 'a', default_value = "./rawaddrman.json")]
    peers: PathBuf,
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    let log_level = args.log_level;
    env_logger::builder().filter_level(log_level).init();

    if let Some(addrman) = Addrman::from_json_file(&args.peers) {
        info!(
            "Got Addrman with {} new buckets and {} tried buckets",
            addrman.new_table.len(),
            addrman.tried_table.len()
        );
    }
}
