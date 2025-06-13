use log::info;
use serde::Deserialize;
use std::collections::HashMap;

pub(crate) type Addr = String;
pub(crate) type Bucket = usize;
pub(crate) type Position = usize;

#[derive(Debug, Default)]
pub struct Addrman {
    pub new_table: HashMap<Bucket, HashMap<Addr, Position>>,
    pub tried_table: HashMap<Bucket, HashMap<Addr, Position>>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawAddrman {
    pub(crate) new: HashMap<String, AddrInfo>,
    pub(crate) tried: HashMap<String, AddrInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AddrInfo {
    pub(crate) address: String,
    pub(crate) port: i64,
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
            let (bucket, position) = split_bucket_pos_str(buc_pos);
            addrman.insert_to_table(true, bucket, position, addrinfo.address);
        }
        for (buc_pos, addrinfo) in raw_addrman.tried {
            let (bucket, position) = split_bucket_pos_str(buc_pos);
            addrman.insert_to_table(false, bucket, position, addrinfo.address);
        }
        addrman
    }

    fn insert_to_table(&mut self, new: bool, bucket: usize, position: usize, address: String) {
        let entry = if new {
            self.new_table.get_mut(&bucket)
        } else {
            self.tried_table.get_mut(&bucket)
        };
        if let Some(entry) = entry {
            entry.insert(address, position);
        } else {
            if new {
                self.new_table
                    .insert(bucket, HashMap::from([(address, position)]));
            } else {
                self.tried_table
                    .insert(bucket, HashMap::from([(address, position)]));
            }
        }
    }
}
fn split_bucket_pos_str(buc_pos: String) -> (Bucket, Position) {
    let parts = buc_pos.split('/').collect::<Vec<_>>();
    let bucket = parts[0].parse::<usize>().unwrap_or_default();
    let position = parts[1].parse::<usize>().unwrap_or_default();
    (bucket, position)
}

#[cfg(test)]
mod tests {}
