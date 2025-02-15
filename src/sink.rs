use crate::commons::Log;
use crate::utils;
use std::time::Duration;

pub struct SinkConfig {
    retry_policy: RetryPolicy,
    sink_type: SinkType,
}

pub trait SleepPolicy {
    fn sleep_for(&self, retry_count: u64) -> Duration;
}

struct ExponetialBackoff {
    initial_delay_ms: u64,
    exp_factor: u64,
}

impl SleepPolicy for ExponetialBackoff {
    fn sleep_for(&self, retry_count: u64) -> Duration {
        let duration = utils::exponent(self.exp_factor, retry_count);
        Duration::from_millis(self.initial_delay_ms * duration)
    }
}

pub struct RetryPolicy {
    max_retrys: i32,
    sleep_policy: Box<dyn SleepPolicy>,
}

pub enum SinkType {
    Kafka,
    MySQLDatabase,
}

pub trait Sink {
    fn flush(log: Log);
}
