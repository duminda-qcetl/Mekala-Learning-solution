pub mod rate_limiter {
    use actix_web::{dev::ServiceRequest, Error, HttpResponse};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    #[derive(Clone)]
    pub struct RateLimiter {
        requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
        max_requests: usize,
        window: Duration,
    }

    impl RateLimiter {
        pub fn new(max_requests: usize, window_secs: u64) -> Self {
            Self {
                requests: Arc::new(Mutex::new(HashMap::new())),
                max_requests,
                window: Duration::from_secs(window_secs),
            }
        }

        pub fn check_rate_limit(&self, key: &str) -> bool {
            let mut requests = self.requests.lock().unwrap();
            let now = Instant::now();
            
            let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);
            entry.retain(|&time| now.duration_since(time) < self.window);
            
            if entry.len() < self.max_requests {
                entry.push(now);
                true
            } else {
                false
            }
        }
    }
}
