use serde::{Deserialize, Serialize};
use std::fs;

pub const WAL_MAX_SEGMENTS: u64 = 5;
pub const MEMTABLE_MAX_SEGMENTS: u64 = 5;
pub const LSM_MAX_LEVELS: u64 = 5;
pub const LSM_MAX_ELEMENTS_PER_LEVEL: u64 = 5;
pub const CACHE_MAX_ELEMENTS: u64 = 5;
pub const TOKEN_BUCKET_MAX_ELEMENTS: u64 = 5;
pub const TOKEN_BUCKET_INTERVAL_IN_SECS: u64 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "wal_max_segments")]
    pub wal_max_segments: u64,
    #[serde(default = "memtable_max_elements")]
    pub memtable_max_elements: u64,
    #[serde(default = "lsm_max_levels")]
    pub lsm_max_levels: u64,
    #[serde(default = "lsm_max_elements_per_level")]
    pub lsm_max_elements_per_level: u64,
    #[serde(default = "cache_max_elements")]
    pub cache_max_elements: u64,
    #[serde(default = "token_bucket_max_elements")]
    pub token_bucket_max_elements: u64,
    #[serde(default = "token_bucket_interval_in_secs")]
    pub token_bucket_interval_in_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            wal_max_segments: WAL_MAX_SEGMENTS,
            memtable_max_elements: MEMTABLE_MAX_SEGMENTS,
            lsm_max_levels: LSM_MAX_LEVELS,
            lsm_max_elements_per_level: LSM_MAX_ELEMENTS_PER_LEVEL,
            cache_max_elements: CACHE_MAX_ELEMENTS,
            token_bucket_max_elements: TOKEN_BUCKET_MAX_ELEMENTS,
            token_bucket_interval_in_secs: TOKEN_BUCKET_INTERVAL_IN_SECS,
        }
    }
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        match fs::read_to_string(file_path) {
            Ok(json_data) => serde_json::from_str(&json_data).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }
}

fn wal_max_segments() -> u64 {
    WAL_MAX_SEGMENTS
}

fn memtable_max_elements() -> u64 {
    MEMTABLE_MAX_SEGMENTS
}

fn lsm_max_levels() -> u64 {
    LSM_MAX_LEVELS
}

fn lsm_max_elements_per_level() -> u64 {
    LSM_MAX_ELEMENTS_PER_LEVEL
}

fn cache_max_elements() -> u64 {
    CACHE_MAX_ELEMENTS
}

fn token_bucket_max_elements() -> u64 {
    TOKEN_BUCKET_MAX_ELEMENTS
}

fn token_bucket_interval_in_secs() -> u64 {
    TOKEN_BUCKET_INTERVAL_IN_SECS
}
