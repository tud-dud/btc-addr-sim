use log::{error, info};
use std::{fs, path::PathBuf};

use crate::types::{AddrInfo, Addrman, RawAddrman};

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
