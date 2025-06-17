use log::{debug, error, info};
use rand::prelude::*;
use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::types::{Addr, AddrInfo, Addrman, RawAddrman};

impl Addrman {
    pub fn from_json_file(path: &PathBuf) -> Option<Self> {
        info!("Reading {}.", path.display());
        let addrman = if let Ok(json_str) = fs::read_to_string(path) {
            Some(Self::from_raw(RawAddrman::from_json_str(&json_str)))
        } else {
            error!("Error reading file to string.");
            None
        };
        addrman
    }
    pub fn insert_attacker_addresses(&mut self, addrs: &Vec<(AddrInfo, u16, u16)>) {
        info!("Inserting {} addresses in addrman.", addrs.len());
        for (addr, bucket, position) in addrs {
            // check and remove if there's an address in the position
            self.insert_to_table(true, *bucket, *position, addr.address.clone());
        }
        info!(
            "Updated Addrman has {} new buckets and {} tried buckets",
            self.new_table.len(),
            self.tried_table.len()
        );
    }
    // removes this address and any referenes that may exists
    pub(crate) fn move_from_new(addr: &Addr) {
        debug!("Removing {} from new", addr);
    }

    pub fn get_initial_connections(&mut self, seed: u64, number: usize) {
        let mut rng = SmallRng::seed_from_u64(seed);
        for (table, new) in [
            (self.new_table.clone(), true),
            (self.tried_table.clone(), false),
        ] {
            let chosen = table.into_iter().choose_multiple(&mut rng, number / 2);
            self.init_peers(new, chosen);
        }
    }

    /// Adds them to list of current peers and removes them from the table
    fn init_peers(&mut self, new: bool, chosen: Vec<(u16, BTreeMap<u16, String>)>) {
        for conn in chosen.iter() {
            let addr = conn
                .1
                .first_key_value()
                .unwrap_or((&u16::default(), &String::default()))
                .1
                .to_owned();
            if let Some((bucket, position)) = self.get_pos_in_table(new, &addr) {
                // remove from table
                self.remove_from_table(new, bucket, position);
            }
            self.current_peers.push(addr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::types::Addrman;

    #[test]
    fn addrman_from_file() {
        let path_to_file = Path::new("./test_data/rawaddrman.json");
        let actual = Addrman::from_json_file(&path_to_file.to_path_buf());
        assert!(actual.is_some());
    }
}
