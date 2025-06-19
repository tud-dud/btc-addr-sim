use log::{debug, info, trace};
use rand::prelude::*;

use crate::{
    helpers,
    types::{Addr, Addrman},
    NUM_BUCKETS_TRIED, NUM_POSITIONS,
};

const FEELER_TIME_INTERVAL: u64 = 2;
const REPLAY_TIME_INTERVAL: u64 = 4;

#[derive(Debug, Default)]
pub struct Simulation {
    pub addrman: Addrman,
    pub attacker_addrs: Vec<Addr>,
    /// the current simulation minute
    pub steps: u64,
    /// Stop simulation after this many steps. 0 means forever
    pub stop_at: u64,
    pub seed: u64,
    /// the max number of connections to terminate in each round
    pub concurrency: usize,
}

impl Simulation {
    pub fn new(
        addrman: Addrman,
        stop_at: u64,
        num_attacker: usize,
        seed: u64,
        concurrency: usize,
    ) -> Self {
        let attacker_addrs = helpers::generate_attacker_addresses(num_attacker, seed);
        let mut addrman = Addrman { ..addrman };
        addrman.insert_attacker_addresses(&attacker_addrs);
        let mut attacker_addrs: Vec<String> =
            attacker_addrs.iter().map(|a| a.0.address.clone()).collect();
        attacker_addrs.sort_unstable();
        Self {
            addrman,
            attacker_addrs,
            steps: 1,
            stop_at,
            seed,
            concurrency,
        }
    }

    /// returns a list of (time, #num_connections) which is updated whenever the attacker gets a
    /// new connection
    pub(crate) fn start(&mut self) -> Vec<(u64, u64)> {
        info!(
            "Running simulation with {} attacker addresses {} and concurrency factor of {}.",
            self.attacker_addrs.len(),
            if self.stop_at > 0 {
                format!("for {} rounds", self.stop_at)
            } else {
                "forever".to_owned()
            },
            self.concurrency,
        );
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut stop = false;
        let mut att_count_by_step = vec![];
        while !stop {
            trace!("Current round = {}", self.steps);
            if self.stop_at > 0 && self.steps >= self.stop_at {
                stop = true;
            }
            if self.steps % FEELER_TIME_INTERVAL == 0 {
                self.make_feeler_connection(&mut rng);
            }
            if self.steps % REPLAY_TIME_INTERVAL == 0 {
                self.replay_packet(&mut rng);
                for _ in 0..self.concurrency {
                    if self.choose_new_peer(&mut rng) {
                        let num_conns = if let Some((_, prev_cnt)) = att_count_by_step.last() {
                            prev_cnt + 1
                        } else {
                            1
                        };
                        att_count_by_step.push((self.steps, num_conns));
                    }
                }
            }
            if self.is_eclipsed() {
                info!(
                    "[ROUND = {}] Eclipsed by {} attacker addresses",
                    self.steps,
                    self.addrman.current_peers.len(),
                );
                stop = true;
            }
            self.steps += 1;
            if self.steps % 100 == 0 {
                info!(
                    "[ROUND = {}] Current number of attacker connections: {}",
                    self.steps,
                    self.current_num_attacker_connections()
                );
            }
        }
        info!("Finished simulation after {} rounds", self.steps);
        att_count_by_step
    }

    fn make_feeler_connection(&mut self, rng: &mut SmallRng) {
        // 1. pick random bucket and address from new table
        let new_table = self.addrman.new_table.clone();
        if let Some(rand_bucket) = new_table.iter().choose(rng) {
            if let Some(rand_addr) = rand_bucket.1.iter().choose(rng) {
                debug!(
                    "[ROUND = {}] Making feeler connection to {}",
                    self.steps, rand_addr.1
                );
                // 2. move to tried
                let bucket: u16 = rng.random_range(..NUM_BUCKETS_TRIED);
                let pos: u16 = rng.random_range(..NUM_POSITIONS);
                self.addrman
                    .move_from_new_to_tried(rand_addr.1, bucket, pos);
            }
        }
    }

    fn replay_packet(&mut self, rng: &mut SmallRng) {
        // 0. ignore attacker addresses
        let mut potential_addresses = self.addrman.current_peers.clone();
        potential_addresses.retain(|peer| !self.attacker_addrs.contains(peer));
        // 1. pick random peer to terminate
        let to_close = potential_addresses.choose_multiple(rng, self.concurrency);
        for replayed_conn in to_close {
            debug!(
                "[ROUND = {}] Connection to {} was closed",
                self.steps, replayed_conn
            );
            self.addrman
                .current_peers
                .retain(|peer| *peer != *replayed_conn);
            // 2. remove this address from all tables because the attacker never allows it again
            if let Some((bucket, position)) = self.addrman.get_pos_in_table(true, replayed_conn) {
                self.addrman.remove_all_from_table(true, bucket, position);
            } else if let Some((bucket, position)) =
                self.addrman.get_pos_in_table(false, replayed_conn)
            {
                self.addrman.remove_all_from_table(false, bucket, position);
            }
        }
    }
    fn choose_new_peer(&mut self, rng: &mut SmallRng) -> bool {
        if self.addrman.current_peers.len() >= 10 {
            return false;
        }
        let mut new_peer_is_attacker = false;
        // 1. choose a table
        if let Some(use_new_table) = [true, false].choose(rng) {
            debug!(
                "[ROUND = {}] New connection will be chosen from {} table",
                self.steps,
                if *use_new_table { "new" } else { "tried" },
            );
            let table = if *use_new_table {
                self.addrman.new_table.clone()
            } else {
                self.addrman.tried_table.clone()
            };
            // 2. choose random address to connect to
            if let Some(rand_bucket) = table.iter().choose(rng) {
                if let Some(rand_addr) = rand_bucket.1.iter().choose(rng) {
                    debug!(
                        "[ROUND = {}] Establishing outgoing connection to {}",
                        self.steps, rand_addr.1
                    );
                    // 3. add it to current peers
                    self.addrman.current_peers.push(rand_addr.1.clone());
                    // 4. remove this address from the tables
                    if let Some((bucket, position)) =
                        self.addrman.get_pos_in_table(true, rand_addr.1)
                    {
                        self.addrman.remove_all_from_table(true, bucket, position);
                    } else if let Some((bucket, position)) =
                        self.addrman.get_pos_in_table(false, rand_addr.1)
                    {
                        self.addrman.remove_all_from_table(false, bucket, position);
                    }
                    if self.is_attacker_address(rand_addr.1) {
                        new_peer_is_attacker = true;
                    }
                }
            }
        }
        new_peer_is_attacker
    }

    fn is_eclipsed(&self) -> bool {
        let mut is_eclipsed = true;
        if self.addrman.current_peers.len() < 10 {
            is_eclipsed = false;
        } else {
            for p in self.addrman.current_peers.iter() {
                if !self.is_attacker_address(p) {
                    is_eclipsed = false;
                    break;
                }
            }
        }
        is_eclipsed
    }
    fn current_num_attacker_connections(&self) -> usize {
        let mut attacker = 0;
        for peer in self.addrman.current_peers.iter() {
            if self.is_attacker_address(peer) {
                attacker += 1;
            }
        }
        attacker
    }
    fn is_attacker_address(&self, addr: &Addr) -> bool {
        self.attacker_addrs.contains(addr)
    }
}
