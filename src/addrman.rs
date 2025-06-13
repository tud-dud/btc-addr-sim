use log::{error, info};
use std::{fs, path::PathBuf};

use crate::types::{Addrman, RawAddrman};

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
