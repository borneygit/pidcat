use crate::filter::Filter;
use crate::log::{Level, Log};
use async_trait::async_trait;

pub struct LevelFilter {
    level: Level,
}

impl LevelFilter {
    #[allow(dead_code)]
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

#[async_trait]
impl Filter for LevelFilter {
    async fn filter(&self, log: &Log) -> bool {
        use std::str::FromStr;

        if let Ok(level) = Level::from_str(&log.level) {
            if level >= self.level {
                return false;
            }
        }
        true
    }
}
