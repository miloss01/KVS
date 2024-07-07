use crate::{
    Config, CACHE_MAX_ELEMENTS, LSM_MAX_ELEMENTS_PER_LEVEL, LSM_MAX_LEVELS, MEMTABLE_MAX_SEGMENTS,
    TOKEN_BUCKET_INTERVAL_IN_SECS, TOKEN_BUCKET_MAX_ELEMENTS, WAL_MAX_SEGMENTS,
};

#[test]
fn test_loading_wrong_path_json() {
    let config: Config = Config::new("nonexistent.json");

    assert_eq!(config.wal_max_segments, WAL_MAX_SEGMENTS);
    assert_eq!(config.memtable_max_elements, MEMTABLE_MAX_SEGMENTS);
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

    assert_eq!(config.wal_max_segments, 1);
    assert_eq!(config.memtable_max_elements, 2);
    assert_eq!(config.lsm_max_levels, 3);
    assert_eq!(config.lsm_max_elements_per_level, 4);
    assert_eq!(config.cache_max_elements, 5);
    assert_eq!(config.token_bucket_max_elements, 6);
    assert_eq!(config.token_bucket_interval_in_secs, 7);
}
