use crate::{
    Config, CACHE_MAX_ELEMENTS, DATA_PATH, LSM_MAX_ELEMENTS_PER_LEVEL, LSM_MAX_LEVELS,
    MEMTABLE_MAX_SEGMENTS, NTH_ELEMENT_IN_SUMMARY, SKIPLIST_MAX_LEVEL, SKIPLIST_PROBABILITY,
    TOKEN_BUCKET_INTERVAL_IN_SECS, TOKEN_BUCKET_MAX_ELEMENTS, WAL_MAX_ELEMENTS, WAL_MAX_SEGMENTS,
    WAL_PATH,
};

#[test]
fn test_loading_wrong_path_json() {
    let config: Config = Config::new("nonexistent.json");

    assert_eq!(config.wal_path, WAL_PATH);
    assert_eq!(config.wal_max_segments, WAL_MAX_SEGMENTS);
    assert_eq!(config.wal_max_elements, WAL_MAX_ELEMENTS);
    assert_eq!(config.data_path, DATA_PATH);
    assert_eq!(config.memtable_max_elements, MEMTABLE_MAX_SEGMENTS);
    assert_eq!(config.skiplist_max_level, SKIPLIST_MAX_LEVEL);
    assert_eq!(config.skiplist_probability, SKIPLIST_PROBABILITY);
    assert_eq!(config.nth_element_in_summary, NTH_ELEMENT_IN_SUMMARY);
    assert_eq!(config.lsm_max_levels, LSM_MAX_LEVELS);
    assert_eq!(
        config.lsm_max_elements_per_level,
        LSM_MAX_ELEMENTS_PER_LEVEL
    );
    assert_eq!(config.cache_max_elements, CACHE_MAX_ELEMENTS);
    assert_eq!(config.token_bucket_max_elements, TOKEN_BUCKET_MAX_ELEMENTS);
    assert_eq!(
        config.token_bucket_interval_in_secs,
        TOKEN_BUCKET_INTERVAL_IN_SECS
    );
}

#[test]
fn test_half_populated_json() {
    let config: Config = Config::new("test_data/half_populated.json");

    assert_eq!(config.wal_max_segments, 3);
    assert_eq!(config.memtable_max_elements, 3);
    assert_eq!(config.lsm_max_levels, 6);
    assert_eq!(config.lsm_max_elements_per_level, 2);
    assert_eq!(config.cache_max_elements, CACHE_MAX_ELEMENTS);
    assert_eq!(config.token_bucket_max_elements, TOKEN_BUCKET_MAX_ELEMENTS);
    assert_eq!(
        config.token_bucket_interval_in_secs,
        TOKEN_BUCKET_INTERVAL_IN_SECS
    );
}

#[test]
fn test_fully_populated_json() {
    let config: Config = Config::new("test_data/fully_populated.json");

    assert_eq!(config.wal_path, "data/wal2");
    assert_eq!(config.wal_max_segments, 1);
    assert_eq!(config.wal_max_elements, 15);
    assert_eq!(config.data_path, "data2");
    assert_eq!(config.memtable_max_elements, 2);
    assert_eq!(config.skiplist_max_level, 6);
    assert_eq!(config.skiplist_probability, 0.2);
    assert_eq!(config.nth_element_in_summary, 7);
    assert_eq!(config.lsm_max_levels, 3);
    assert_eq!(config.lsm_max_elements_per_level, 4);
    assert_eq!(config.cache_max_elements, 5);
    assert_eq!(config.token_bucket_max_elements, 6);
    assert_eq!(config.token_bucket_interval_in_secs, 7);
}
