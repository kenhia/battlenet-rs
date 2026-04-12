use std::sync::atomic::{AtomicU32, Ordering};
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// Configuration for the API rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    pub per_second: u32,
    pub per_hour: u32,
    pub nice_mode: bool,
    pub nice_per_second: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            per_second: 100,
            per_hour: 36_000,
            nice_mode: false,
            nice_per_second: 50,
        }
    }
}

/// Dual-window token bucket rate limiter.
pub struct RateLimiter {
    config: RateLimiterConfig,
    second_tokens: AtomicU32,
    second_window_start: Mutex<Instant>,
    hour_tokens: AtomicU32,
    hour_window_start: Mutex<Instant>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimiter")
            .field("config", &self.config)
            .field("second_tokens", &self.second_tokens.load(Ordering::Relaxed))
            .field("hour_tokens", &self.hour_tokens.load(Ordering::Relaxed))
            .finish()
    }
}

impl RateLimiter {
    pub fn new(config: RateLimiterConfig) -> Self {
        let effective_per_second = if config.nice_mode {
            config.nice_per_second
        } else {
            config.per_second
        };
        Self {
            second_tokens: AtomicU32::new(effective_per_second),
            second_window_start: Mutex::new(Instant::now()),
            hour_tokens: AtomicU32::new(config.per_hour),
            hour_window_start: Mutex::new(Instant::now()),
            config,
        }
    }

    /// Wait until a request slot is available, then consume it.
    pub async fn acquire(&self) {
        loop {
            // Try to refill windows if needed
            self.try_refill_second().await;
            self.try_refill_hour().await;

            // Try to consume from both buckets

            // Check per-hour first (longer wait)
            let hour_remaining = self.hour_tokens.load(Ordering::Acquire);
            if hour_remaining == 0 {
                // Wait for the hourly window to reset
                let window_start = self.hour_window_start.lock().await;
                let elapsed = window_start.elapsed();
                drop(window_start);
                let wait = Duration::from_secs(3600).saturating_sub(elapsed);
                if !wait.is_zero() {
                    tokio::time::sleep(wait).await;
                }
                continue;
            }

            // Check per-second
            let second_remaining = self.second_tokens.load(Ordering::Acquire);
            if second_remaining == 0 {
                // Wait for the per-second window to reset
                let window_start = self.second_window_start.lock().await;
                let elapsed = window_start.elapsed();
                drop(window_start);
                let wait = Duration::from_secs(1).saturating_sub(elapsed);
                if !wait.is_zero() {
                    tokio::time::sleep(wait).await;
                }
                continue;
            }

            // Try to atomically consume from both buckets (CAS)
            let prev_s =
                self.second_tokens
                    .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                        if n > 0 {
                            Some(n - 1)
                        } else {
                            None
                        }
                    });

            if prev_s.is_err() {
                continue; // Another task consumed the tokens, retry
            }

            let prev_h = self
                .hour_tokens
                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                    if n > 0 {
                        Some(n - 1)
                    } else {
                        None
                    }
                });

            if prev_h.is_err() {
                // Give back the per-second token we consumed
                self.second_tokens.fetch_add(1, Ordering::Release);
                continue;
            }

            return;
        }
    }

    /// Check current remaining per-second tokens.
    pub fn remaining_per_second(&self) -> u32 {
        self.second_tokens.load(Ordering::Acquire)
    }

    /// Check current remaining per-hour tokens.
    pub fn remaining_per_hour(&self) -> u32 {
        self.hour_tokens.load(Ordering::Acquire)
    }

    async fn try_refill_second(&self) {
        let mut window_start = self.second_window_start.lock().await;
        if window_start.elapsed() >= Duration::from_secs(1) {
            let effective = if self.config.nice_mode {
                self.config.nice_per_second
            } else {
                self.config.per_second
            };
            self.second_tokens.store(effective, Ordering::Release);
            *window_start = Instant::now();
        }
    }

    async fn try_refill_hour(&self) {
        let mut window_start = self.hour_window_start.lock().await;
        if window_start.elapsed() >= Duration::from_secs(3600) {
            self.hour_tokens
                .store(self.config.per_hour, Ordering::Release);
            *window_start = Instant::now();
        }
    }
}
