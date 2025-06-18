use log::{info, warn};
use rand::prelude::*;

use crate::{
    types::{AddrInfo, Bucket, Position},
    NUM_BUCKETS_NEW, NUM_POSITIONS,
};

pub(crate) fn generate_attacker_addresses(num: usize, seed: u64) -> Vec<(AddrInfo, u16, u16)> {
    if num > 256 {
        warn!("Support for >256 /16 subnets not yet available. Defaulting to 256 addresses.")
    }
    let num = num.min(256);
    info!("Generating {num} attacker addresses.");
    let mut addrs = vec![];
    let mut rng = SmallRng::seed_from_u64(seed);
    let port = 18444;
    let net = rng.random_range(1..255);
    for i in 0..num {
        let ip = format!("10.1.{}.{}", i + 1, net);
        // random bucket and random position
        let bucket: u16 = rng.random_range(..NUM_BUCKETS_NEW);
        let pos: u16 = rng.random_range(..NUM_POSITIONS);
        addrs.push((AddrInfo { address: ip, port }, bucket, pos));
    }
    addrs
}

pub(crate) fn split_bucket_pos_str(buc_pos: String) -> (Bucket, Position) {
    let parts = buc_pos.split('/').collect::<Vec<_>>();
    let bucket = parts[0].parse::<u16>().unwrap_or_default();
    let position = parts[1].parse::<u16>().unwrap_or_default();
    (bucket, position)
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
                    address: "10.1.1.207".to_string(),
                    port: 18444,
                },
                765,
                6,
            ),
            (
                AddrInfo {
                    address: "10.1.2.207".to_string(),
                    port: 18444,
                },
                764,
                11,
            ),
        ];
        assert_eq!(actual, expected);
    }
}
