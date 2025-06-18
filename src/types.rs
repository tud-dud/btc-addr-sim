use log::{debug, info};
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::helpers;
pub(crate) type Addr = String;
pub(crate) type Bucket = u16;
pub(crate) type Position = u16;

#[derive(Debug, Default)]
pub struct Addrman {
    /// has 1024 buckets with 64 slots
    pub new_table: BTreeMap<Bucket, BTreeMap<Position, Addr>>,
    /// has 256 buckets with 64 slots
    pub tried_table: BTreeMap<Bucket, BTreeMap<Position, Addr>>,
    pub current_peers: Vec<Addr>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawAddrman {
    pub(crate) new: BTreeMap<String, AddrInfo>,
    pub(crate) tried: BTreeMap<String, AddrInfo>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AddrInfo {
    pub address: String,
    pub port: u16,
}

impl RawAddrman {
    pub(crate) fn from_json_str(json_str: &str) -> Self {
        serde_json::from_str(json_str).expect("error")
    }
}

impl Addrman {
    pub(crate) fn from_raw(raw_addrman: RawAddrman) -> Self {
        info!(
            "Parsing {} new and {} tried entries",
            raw_addrman.new.len(),
            raw_addrman.tried.len()
        );
        let mut addrman = Addrman::default();
        for (buc_pos, addrinfo) in raw_addrman.new {
            let (bucket, position) = helpers::split_bucket_pos_str(buc_pos);
            addrman.insert_to_table(true, bucket, position, addrinfo.address);
        }
        for (buc_pos, addrinfo) in raw_addrman.tried {
            let (bucket, position) = helpers::split_bucket_pos_str(buc_pos);
            addrman.insert_to_table(false, bucket, position, addrinfo.address);
        }
        addrman
    }

    pub(crate) fn insert_to_table(
        &mut self,
        new: bool,
        bucket: u16,
        position: u16,
        address: String,
    ) {
        let entry = if new {
            self.new_table.get_mut(&bucket)
        } else {
            self.tried_table.get_mut(&bucket)
        };
        if let Some(entry) = entry {
            entry.insert(position, address);
        } else if new {
            self.new_table
                .insert(bucket, BTreeMap::from([(position, address)]));
        } else {
            self.tried_table
                .insert(bucket, BTreeMap::from([(position, address)]));
        }
    }
    // returns the bucket and position, if available
    pub(crate) fn get_pos_in_table(&self, new: bool, address: &Addr) -> Option<(Bucket, Position)> {
        let table = if new {
            self.new_table.clone()
        } else {
            self.tried_table.clone()
        };
        for bucket in table.iter() {
            for (position, addr) in bucket.1 {
                if addr == address {
                    return Some((*bucket.0, *position));
                }
            }
        }
        None
    }

    // TODO: Duplication
    pub(crate) fn remove_all_from_table(&mut self, new: bool, bucket: u16, position: u16) {
        let mut occurences = 0;
        let table = if new { "new" } else { "tried" };
        if new {
            if let Some(bucket) = self.new_table.get_mut(&bucket) {
                if let Some(addr) = bucket.remove(&position) {
                    occurences += 1;
                    while let Some((bucket, position)) = self.get_pos_in_table(true, &addr) {
                        self.new_table.get_mut(&bucket).unwrap().remove(&position);
                        occurences += 1;
                    }
                }
            }
        } else if let Some(bucket) = self.tried_table.get_mut(&bucket) {
            if let Some(addr) = bucket.remove(&position) {
                occurences += 1;
                while let Some((bucket, position)) = self.get_pos_in_table(true, &addr) {
                    self.tried_table.get_mut(&bucket).unwrap().remove(&position);
                    occurences += 1;
                }
            }
        }
        debug!("removed {} occurences from {} table", occurences, table);
    }
}

#[cfg(test)]
mod tests {}
