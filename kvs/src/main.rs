use kvs_lib::LRUCache;

fn main() {
    let mut lru_cache: LRUCache = LRUCache::new(2);

    // lru_cache.put("key1".to_string(), vec![1, 2, 3]);
    // lru_cache.put("key2".to_string(), vec![4, 5, 6]);

    // println!("{:?}", lru_cache.get("key1")); // Some([1, 2, 3])
    // println!("{:?}", lru_cache.get("key3")); // None

    // lru_cache.put("key3".to_string(), vec![7, 8, 9]);

    // println!("{:?}", lru_cache.get("key2")); // None (key2 should be evicted)
    // println!("{:?}", lru_cache.get("key3")); // Some([7, 8, 9])
}
