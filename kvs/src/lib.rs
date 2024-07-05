pub(crate) mod structures {
    pub(crate) mod lru_cache {
        pub(crate) mod lru_cache;

        #[cfg(test)]
        mod test;
    }
}

pub use structures::lru_cache::lru_cache::LRUCache;
