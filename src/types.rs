use csv::Writer;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

use crate::{helpers, sim::Simulation};
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

#[derive(Debug, PartialEq, Serialize)]
pub struct SimOutput {
    pub run: u64,
    pub concurrency: usize,
    pub num_prefixes: usize,
    pub total_hours: f64,
    pub slot_1: u64,
    pub slot_2: u64,
    pub slot_3: u64,
    pub slot_4: u64,
    pub slot_5: u64,
    pub slot_6: u64,
    pub slot_7: u64,
    pub slot_8: u64,
    pub slot_9: u64,
    pub slot_10: u64,
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

impl SimOutput {
    /// expects a list of pairs
    pub fn from_sim(sim: &Simulation, time_conns: &[(u64, u64)]) -> Self {
        Self {
            run: sim.seed,
            concurrency: sim.concurrency,
            num_prefixes: sim.attacker_addrs.len(),
            total_hours: sim.steps as f64 / 60.0,
            slot_1: time_conns[0].0,
            slot_2: time_conns[1].0,
            slot_3: time_conns[2].0,
            slot_4: time_conns[3].0,
            slot_5: time_conns[4].0,
            slot_6: time_conns[5].0,
            slot_7: time_conns[6].0,
            slot_8: time_conns[7].0,
            slot_9: time_conns[8].0,
            slot_10: time_conns[9].0,
        }
    }

    pub fn to_file(&self, path: PathBuf) {
        let mut path = path.clone();
        path.push(format!(
            "sim-s{}-n{}-c{}.csv",
            self.run, self.num_prefixes, self.concurrency
        ));
        let mut writer = Writer::from_path(path).expect("Error creating csv writer");
        writer.serialize(self).expect("Error serializing SimOutput");
    }
}

#[cfg(test)]
mod tests {}
