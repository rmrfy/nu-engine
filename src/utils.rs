use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in milliseconds, like Java's System.currentTimeMillis().
pub fn current_time_millis() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as i64,
        Err(error) => -(error.duration().as_millis() as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_time_is_in_unix_milliseconds() {
        let before = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let timestamp = current_time_millis();
        let after = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        assert!(timestamp >= before * 1_000);
        assert!(timestamp < (after + 1) * 1_000);
    }
}
