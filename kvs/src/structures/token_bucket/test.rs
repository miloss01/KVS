use crate::TokenBucket;
use std::thread;
use std::time::Duration;

#[test]
fn test_to_empty_bucket() {
    let mut token_bucket: TokenBucket = TokenBucket::new(4, 3);

    assert!(token_bucket.use_token());
    assert!(token_bucket.use_token());
    assert!(token_bucket.use_token());
    assert!(token_bucket.use_token());

    assert!(!token_bucket.use_token());
    assert!(!token_bucket.use_token());
    assert!(!token_bucket.use_token());
}

#[test]
fn test_to_empty_to_half_and_try_after_fill() {
    let mut token_bucket: TokenBucket = TokenBucket::new(2, 3);

    assert!(token_bucket.use_token());
    assert!(token_bucket.use_token());

    assert!(!token_bucket.use_token());

    thread::sleep(Duration::from_secs(4));
    assert!(token_bucket.use_token());
    assert!(token_bucket.use_token());

    assert!(!token_bucket.use_token());
}
