use crate::LRUCache;

#[test]
fn test_lru_cache_put_and_get() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1".to_string().into_bytes(), vec![1, 2, 3]);
    lru_cache.put("key2".to_string().into_bytes(), vec![4, 5, 6]);
    lru_cache.put("key3".to_string().into_bytes(), vec![7, 8, 9]);

    assert_eq!(
        lru_cache.get("key1".to_string().into_bytes()),
        Some(&vec![1, 2, 3])
    );
    assert_eq!(
        lru_cache.get("key2".to_string().into_bytes()),
        Some(&vec![4, 5, 6])
    );
    assert_eq!(
        lru_cache.get("key3".to_string().into_bytes()),
        Some(&vec![7, 8, 9])
    );
}

#[test]
fn test_eviction() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1".to_string().into_bytes(), vec![1, 2, 3]);
    lru_cache.put("key2".to_string().into_bytes(), vec![4, 5, 6]);
    lru_cache.put("key3".to_string().into_bytes(), vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key1".to_string().into_bytes()), None); // key1 should be evicted
    assert_eq!(
        lru_cache.get("key2".to_string().into_bytes()),
        Some(&vec![4, 5, 6])
    );
    assert_eq!(
        lru_cache.get("key3".to_string().into_bytes()),
        Some(&vec![7, 8, 9])
    );
}

#[test]
fn test_update_order() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1".to_string().into_bytes(), vec![1, 2, 3]);
    lru_cache.put("key2".to_string().into_bytes(), vec![4, 5, 6]);

    assert_eq!(
        lru_cache.get("key1".to_string().into_bytes()),
        Some(&vec![1, 2, 3])
    ); // Access key1 to update order
    lru_cache.put("key3".to_string().into_bytes(), vec![7, 8, 9]);

    assert_eq!(lru_cache.get("key2".to_string().into_bytes()), None); // key2 should be evicted
    assert_eq!(
        lru_cache.get("key1".to_string().into_bytes()),
        Some(&vec![1, 2, 3])
    );
    assert_eq!(
        lru_cache.get("key3".to_string().into_bytes()),
        Some(&vec![7, 8, 9])
    );
}

#[test]
fn test_update_value() {
    let mut lru_cache: LRUCache = LRUCache::new(2);
    lru_cache.put("key1".to_string().into_bytes(), vec![1, 2, 3]);
    lru_cache.put("key2".to_string().into_bytes(), vec![4, 5, 6]);

    assert_eq!(
        lru_cache.get("key1".to_string().into_bytes()),
        Some(&vec![1, 2, 3])
    );

    lru_cache.put("key1".to_string().into_bytes(), vec![7, 8, 9]);

    assert_eq!(
        lru_cache.get("key1".to_string().into_bytes()),
        Some(&vec![7, 8, 9])
    ); // Updated value
    assert_eq!(
        lru_cache.get("key2".to_string().into_bytes()),
        Some(&vec![4, 5, 6])
    );
}

#[test]
fn test_access_order() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1".to_string().into_bytes(), vec![1]);
    lru_cache.put("key2".to_string().into_bytes(), vec![2]);
    lru_cache.put("key3".to_string().into_bytes(), vec![3]);

    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key1".to_string().into_bytes(),
            "key2".to_string().into_bytes(),
            "key3".to_string().into_bytes()
        ]
    );

    // Access key2
    lru_cache.get("key2".to_string().into_bytes());
    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key1".to_string().into_bytes(),
            "key3".to_string().into_bytes(),
            "key2".to_string().into_bytes()
        ]
    );

    // Access key1
    lru_cache.get("key1".to_string().into_bytes());
    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key3".to_string().into_bytes(),
            "key2".to_string().into_bytes(),
            "key1".to_string().into_bytes()
        ]
    );

    // Access key3
    lru_cache.get("key3".to_string().into_bytes());
    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key2".to_string().into_bytes(),
            "key1".to_string().into_bytes(),
            "key3".to_string().into_bytes()
        ]
    );

    // Access key2 again
    lru_cache.get("key2".to_string().into_bytes());
    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key1".to_string().into_bytes(),
            "key3".to_string().into_bytes(),
            "key2".to_string().into_bytes()
        ]
    );
}

#[test]
fn test_remove() {
    let mut lru_cache: LRUCache = LRUCache::new(3);

    lru_cache.put("key1".to_string().into_bytes(), vec![1]);
    lru_cache.put("key2".to_string().into_bytes(), vec![2]);
    lru_cache.put("key3".to_string().into_bytes(), vec![3]);

    lru_cache.remove("key1".to_string().into_bytes());
    lru_cache.remove("key2".to_string().into_bytes());

    assert_eq!(lru_cache.get("key1".to_string().into_bytes()), None);
    assert_eq!(lru_cache.get("key2".to_string().into_bytes()), None);
    assert_eq!(
        lru_cache.get("key3".to_string().into_bytes()),
        Some(&vec![3])
    );

    lru_cache.put("key1".to_string().into_bytes(), vec![1]);
    lru_cache.put("key2".to_string().into_bytes(), vec![2]);

    lru_cache.remove("key1".to_string().into_bytes());

    assert_eq!(lru_cache.get("key1".to_string().into_bytes()), None);
    assert_eq!(
        lru_cache.get("key2".to_string().into_bytes()),
        Some(&vec![2])
    );
    assert_eq!(
        lru_cache.get("key3".to_string().into_bytes()),
        Some(&vec![3])
    );

    assert_eq!(
        lru_cache.get_order(),
        vec![
            "key2".to_string().into_bytes(),
            "key3".to_string().into_bytes()
        ]
    );
}
