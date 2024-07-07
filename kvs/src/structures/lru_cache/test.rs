use crate::LRUCache;

#[test]
fn test_lru_cache_put_and_get() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1", vec![1, 2, 3]);
    lru_cache.put("key2", vec![4, 5, 6]);
    lru_cache.put("key3", vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key1"), Some(&vec![1, 2, 3]));
    assert_eq!(lru_cache.get("key2"), Some(&vec![4, 5, 6]));
    assert_eq!(lru_cache.get("key3"), Some(&vec![7, 8, 9]));
}

#[test]
fn test_eviction() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1", vec![1, 2, 3]);
    lru_cache.put("key2", vec![4, 5, 6]);
    lru_cache.put("key3", vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key1"), None); // key1 should be evicted
    assert_eq!(lru_cache.get("key2"), Some(&vec![4, 5, 6]));
    assert_eq!(lru_cache.get("key3"), Some(&vec![7, 8, 9]));
}

#[test]
fn test_update_order() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1", vec![1, 2, 3]);
    lru_cache.put("key2", vec![4, 5, 6]);

    assert_eq!(lru_cache.get("key1"), Some(&vec![1, 2, 3])); // Access key1 to update order
    lru_cache.put("key3", vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key2"), None); // key2 should be evicted
    assert_eq!(lru_cache.get("key1"), Some(&vec![1, 2, 3]));
    assert_eq!(lru_cache.get("key3"), Some(&vec![7, 8, 9]));
}

#[test]
fn test_update_value() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1", vec![1, 2, 3]);
    lru_cache.put("key2", vec![4, 5, 6]);

    assert_eq!(lru_cache.get("key1"), Some(&vec![1, 2, 3]));

    lru_cache.put("key1", vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key1"), Some(&vec![7, 8, 9])); // Updated value
    assert_eq!(lru_cache.get("key2"), Some(&vec![4, 5, 6]));
}

#[test]
fn test_access_order() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1", vec![1]);
    lru_cache.put("key2", vec![2]);
    lru_cache.put("key3", vec![3]);

    assert_eq!(lru_cache.get_order(), vec!["key1", "key2", "key3"]);

    // Access key2
    lru_cache.get("key2");
    assert_eq!(lru_cache.get_order(), vec!["key1", "key3", "key2"]);

    // Access key1
    lru_cache.get("key1");
    assert_eq!(lru_cache.get_order(), vec!["key3", "key2", "key1"]);

    // Access key3
    lru_cache.get("key3");
    assert_eq!(lru_cache.get_order(), vec!["key2", "key1", "key3"]);

    // Access key2 again
    lru_cache.get("key2");
    assert_eq!(lru_cache.get_order(), vec!["key1", "key3", "key2"]);
}

#[test]
fn test_remove() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1", vec![1]);
    lru_cache.put("key2", vec![2]);
    lru_cache.put("key3", vec![3]);

    lru_cache.remove("key1");
    lru_cache.remove("key2");

    assert_eq!(lru_cache.get("key1"), None);
    assert_eq!(lru_cache.get("key2"), None);
    assert_eq!(lru_cache.get("key3"), Some(&vec![3]));
}
