use clap::Parser;
use log::{debug, error, info, LevelFilter};
use rand::prelude::*;
use std::path::PathBuf;
use types::{AddrInfo, Addrman};

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
    /// Number of /16 attacker addresses to generate
    #[arg(long = "num", short = 'n', default_value_t = 8)]
    num_addrs: usize,
    /// Seed for the RNG
    #[arg(long = "seed", short = 's', default_value_t = 999)]
    seed: u64,
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
        let attacker_addrs = generate_attacker_addresses(args.num_addrs, args.seed);
        addrman.insert_attacker_addresses(&attacker_addrs);
    }
}

fn generate_attacker_addresses(num: usize, seed: u64) -> Vec<(AddrInfo, u16, u16)> {
    info!("Generating {num} attacker addresses.");
    let mut addrs = vec![];
    let mut rng = SmallRng::seed_from_u64(seed);
    let port = 18444;
    let net = rng.random_range(1..=255);
    for i in 0..num {
        let ip = format!("11.1.{}.{}", i + 1, net);
        // random bucket and random position
        let bucket: u16 = rng.random_range(..64);
        let pos: u16 = rng.random_range(..1024);
        addrs.push((AddrInfo { address: ip, port }, bucket, pos));
    }
    println!("{:?}", addrs);
    addrs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_gen() {
        let seed = 1;
        let num = 2;
        let actual = generate_attacker_addresses(num, seed);
        assert_eq!(actual.len(), num);
        let expected = vec![
            (
                AddrInfo {
                    address: "11.1.1.207".to_string(),
                    port: 18444,
                },
                47,
                102,
            ),
            (
                AddrInfo {
                    address: "11.1.2.207".to_string(),
                    port: 18444,
                },
                47,
                189,
            ),
        ];
        assert_eq!(actual, expected);
    }
}
