use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenBucket {
    max_tokens: u64,
    remaining_tokens: u64,
    interval: u64,
    next_reset_time: u64,
}

impl TokenBucket {
    pub fn new(max: u64, interval: u64) -> Self {
        TokenBucket {
            max_tokens: max,
            remaining_tokens: max,
            interval,
            next_reset_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64
                + interval,
        }
    }

    pub fn use_token(&mut self) -> bool {
        let current_time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        if current_time > self.next_reset_time {
            self.remaining_tokens = self.max_tokens;
            self.next_reset_time = current_time + self.interval;
        }

        if self.remaining_tokens == 0 {
            return false;
        }

        self.remaining_tokens -= 1;

        true
    }
}
