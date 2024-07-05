use crate::BloomFilter;
use std::fs;

#[test]
fn test_put_and_contains() {
    let mut bloom_filter: BloomFilter = BloomFilter::new(10, 0.1);

    bloom_filter.add("key1");
    bloom_filter.add("key2");
    bloom_filter.add("key3");

    assert!(bloom_filter.contains("key1"));
    assert!(bloom_filter.contains("key2"));
    assert!(bloom_filter.contains("key3"));

    assert!(!bloom_filter.contains("key4"));
    assert!(!bloom_filter.contains("key5"));
}

#[test]
fn test_write_and_load_from_file() {
    let file_path: &str = "test_data/bf_test.dat";

    let mut bloom_filter: BloomFilter = BloomFilter::new(10, 0.1);

    bloom_filter.add("key1");
    bloom_filter.add("key2");
    bloom_filter.add("key3");

    bloom_filter.save_to_file(file_path).unwrap();

    if let Ok(loaded_bloom_filter) = BloomFilter::load_from_file(file_path) {
        assert!(loaded_bloom_filter.contains("key1"));
        assert!(loaded_bloom_filter.contains("key2"));
        assert!(loaded_bloom_filter.contains("key3"));

        fs::remove_file(file_path).unwrap();
    } else {
        assert!(false);
    }
}
