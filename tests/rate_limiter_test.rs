//! Rate limiter unit tests.
//!
//! Tests for the dual-window token bucket rate limiter (per-second + per-hour).

mod rate_limiter_tests {
    use battlenet_rs::rate_limiter::{RateLimiter, RateLimiterConfig};
    use std::time::Instant;

    #[test]
    fn test_config_default() {
        let config = RateLimiterConfig::default();
        assert_eq!(config.per_second, 100);
        assert_eq!(config.per_hour, 36_000);
        assert!(!config.nice_mode);
        assert_eq!(config.nice_per_second, 50);
    }

    #[test]
    fn test_config_custom() {
        let config = RateLimiterConfig {
            per_second: 10,
            per_hour: 1000,
            nice_mode: true,
            nice_per_second: 5,
        };
        assert_eq!(config.per_second, 10);
        assert!(config.nice_mode);
    }

    #[tokio::test]
    async fn test_acquire_within_limit() {
        let config = RateLimiterConfig {
            per_second: 10,
            per_hour: 36_000,
            nice_mode: false,
            nice_per_second: 5,
        };
        let limiter = RateLimiter::new(config);

        // 10 acquires should complete instantly (within the per-second budget)
        let start = Instant::now();
        for _ in 0..10 {
            limiter.acquire().await;
        }
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 500,
            "10 acquires within budget took too long: {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_acquire_exceeding_per_second_throttles() {
        let config = RateLimiterConfig {
            per_second: 5,
            per_hour: 36_000,
            nice_mode: false,
            nice_per_second: 3,
        };
        let limiter = RateLimiter::new(config);

        // Exhaust the 5 per-second budget
        for _ in 0..5 {
            limiter.acquire().await;
        }

        // The 6th acquire should wait for the per-second window to reset (~1s)
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() >= 900,
            "6th acquire should have waited ~1s, waited {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_nice_mode_reduces_rate() {
        let config = RateLimiterConfig {
            per_second: 10,
            per_hour: 36_000,
            nice_mode: true,
            nice_per_second: 3,
        };
        let limiter = RateLimiter::new(config);

        // In nice mode, effective per-second is 3 (not 10)
        for _ in 0..3 {
            limiter.acquire().await;
        }

        // The 4th acquire should throttle
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() >= 900,
            "4th acquire in nice mode should have waited ~1s, waited {:?}",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_remaining_tokens() {
        let config = RateLimiterConfig {
            per_second: 10,
            per_hour: 100,
            nice_mode: false,
            nice_per_second: 5,
        };
        let limiter = RateLimiter::new(config);

        assert_eq!(limiter.remaining_per_second(), 10);
        assert_eq!(limiter.remaining_per_hour(), 100);

        limiter.acquire().await;

        assert_eq!(limiter.remaining_per_second(), 9);
        assert_eq!(limiter.remaining_per_hour(), 99);
    }

    #[tokio::test]
    async fn test_per_hour_limit() {
        let config = RateLimiterConfig {
            per_second: 100,
            per_hour: 5,
            nice_mode: false,
            nice_per_second: 50,
        };
        let limiter = RateLimiter::new(config);

        // Exhaust the hourly budget
        for _ in 0..5 {
            limiter.acquire().await;
        }

        assert_eq!(limiter.remaining_per_hour(), 0);
        // We won't wait for the full hourly reset in a unit test,
        // but we can verify the token count is 0.
    }
}
