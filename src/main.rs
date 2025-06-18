use clap::Parser;
use log::{error, info, LevelFilter};
use sim::Simulation;
use std::path::PathBuf;
use types::{Addrman, SimOutput};

mod addrman;
mod helpers;
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
    #[arg(long = "until", short = 'u', default_value_t = 0)]
    stop_after: u64,
    /// How many regular connections to terminate per round
    #[arg(long = "concurrency", short = 'c', default_value_t = 1)]
    concurrency: usize,
    /// Path to directory where the results will be stored
    #[arg(long = "out", short = 'o')]
    output_dir: Option<PathBuf>,
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    let log_level = args.log_level;
    env_logger::builder().filter_level(log_level).init();

    let output_dir = if let Some(output_dir) = args.output_dir {
        output_dir
    } else {
        PathBuf::from("sim-results")
    };
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        error!("Error creating output directory {}", e);
    }
    info!(
        "Simulation results will be written to {:#?}/ directory.",
        output_dir
    );

    if let Some(mut addrman) = Addrman::from_json_file(&args.peers) {
        info!(
            "Got Addrman with {} new buckets and {} tried buckets",
            addrman.new_table.len(),
            addrman.tried_table.len()
        );
        // + 2 block-relay only peers
        addrman.get_initial_connections(args.seed, 10);
        let mut sim = Simulation::new(
            addrman,
            args.stop_after,
            args.num_addrs,
            args.seed,
            args.concurrency,
        );
        let result = sim.start();
        let sim_output = SimOutput::from_sim(&sim, &result);
        sim_output.to_file(output_dir);
    }
}
