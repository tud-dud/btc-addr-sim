use clap::Parser;
use log::{info, LevelFilter};
use sim::Simulation;
use std::path::PathBuf;
use types::Addrman;

mod addrman;
mod sim;
mod types;

pub const NUM_BUCKETS_NEW: u16 = 1024;
pub const NUM_BUCKETS_TRIED: u16 = 256;
pub const NUM_POSITIONS: u16 = 64;

#[derive(clap::Parser)]
#[command(version, about)]
/// Listen for TCP connections.
struct Cli {
    #[arg(long = "log", short = 'l', default_value = "info")]
    log_level: LevelFilter,
    /// Path to JSON file containing the addrman dump
    #[arg(long = "addrman", short = 'a', default_value = "./rawaddrman.json")]
    peers: PathBuf,
    /// Number of /16 attacker addresses to generate
    #[arg(long = "num", short = 'n', default_value_t = 10)]
    num_addrs: usize,
    /// Seed for the RNG
    #[arg(long = "seed", short = 's', default_value_t = 999)]
    seed: u64,
    /// Stop after this many steps
    #[arg(long = "until", short = 'u')]
    stop_after: Option<u64>,
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    let log_level = args.log_level;
    env_logger::builder().filter_level(log_level).init();

    if let Some(mut addrman) = Addrman::from_json_file(&args.peers) {
        info!(
            "Got Addrman with {} new buckets and {} tried buckets",
            addrman.new_table.len(),
            addrman.tried_table.len()
        );
        // + 2 block-relay only peers
        addrman.get_initial_connections(args.seed, 10);
        let mut sim = Simulation::new(addrman, args.stop_after, args.num_addrs, args.seed);
        sim.start();
    }
}
