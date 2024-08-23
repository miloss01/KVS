use serde::{Deserialize, Serialize};
use std::fs;

pub const WAL_PATH: &str = "data/wal";
pub const WAL_MAX_SEGMENTS: u64 = 5;
pub const WAL_MAX_ELEMENTS: u64 = 10;
pub const DATA_PATH: &str = "data";
pub const MEMTABLE_MAX_SEGMENTS: u64 = 5;
pub const SKIPLIST_MAX_LEVEL: u64 = 5;
pub const SKIPLIST_PROBABILITY: f64 = 0.5;
pub const NTH_ELEMENT_IN_SUMMARY: u64 = 2;
pub const LSM_MAX_LEVELS: u64 = 5;
pub const LSM_MAX_ELEMENTS_PER_LEVEL: u64 = 5;
pub const CACHE_MAX_ELEMENTS: u64 = 5;
pub const TOKEN_BUCKET_MAX_ELEMENTS: u64 = 5;
pub const TOKEN_BUCKET_INTERVAL_IN_SECS: u64 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "wal_path")]
    pub wal_path: String,
    #[serde(default = "wal_max_segments")]
    pub wal_max_segments: u64,
    #[serde(default = "wal_max_elements")]
    pub wal_max_elements: u64,
    #[serde(default = "data_path")]
    pub data_path: String,
    #[serde(default = "memtable_max_elements")]
    pub memtable_max_elements: u64,
    #[serde(default = "skiplist_max_level")]
    pub skiplist_max_level: u64,
    #[serde(default = "skiplist_probability")]
    pub skiplist_probability: f64,
    #[serde(default = "nth_element_in_summary")]
    pub nth_element_in_summary: u64,
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
            wal_path: WAL_PATH.to_string(),
            wal_max_segments: WAL_MAX_SEGMENTS,
            wal_max_elements: WAL_MAX_ELEMENTS,
            data_path: DATA_PATH.to_string(),
            memtable_max_elements: MEMTABLE_MAX_SEGMENTS,
            skiplist_max_level: SKIPLIST_MAX_LEVEL,
            skiplist_probability: SKIPLIST_PROBABILITY,
            nth_element_in_summary: NTH_ELEMENT_IN_SUMMARY,
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

fn wal_path() -> String {
    WAL_PATH.to_string()
}

fn wal_max_segments() -> u64 {
    WAL_MAX_SEGMENTS
}

fn wal_max_elements() -> u64 {
    WAL_MAX_ELEMENTS
}

fn data_path() -> String {
    DATA_PATH.to_string()
}

fn memtable_max_elements() -> u64 {
    MEMTABLE_MAX_SEGMENTS
}

fn skiplist_max_level() -> u64 {
    SKIPLIST_MAX_LEVEL
}

fn skiplist_probability() -> f64 {
    SKIPLIST_PROBABILITY
}

fn nth_element_in_summary() -> u64 {
    NTH_ELEMENT_IN_SUMMARY
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
