use crate::filter::Filter;
use crate::log::{Level, Log};
use async_trait::async_trait;
use std::sync::Arc;

pub(crate) struct LevelFilter {
    filter: Option<Arc<dyn Filter>>,
    level: Level,
}

impl LevelFilter {
    pub fn new(level: Level, filter: Option<Arc<dyn Filter>>) -> Self {
        Self { level, filter }
    }
}

#[async_trait]
impl Filter for LevelFilter {
    async fn filter(&self, mut log: Log) -> Option<Log> {
        if let Some(f) = &self.filter {
            let f = Arc::clone(f);
            if let Some(r) = f.filter(log).await {
                log = r;
            } else {
                return None;
            }
        }

        use std::str::FromStr;

        if let Ok(level) = Level::from_str(&log.level) {
            if level >= self.level {
                Some(log)
            } else {
                None
            }
        } else {
            None
        }
    }
}
