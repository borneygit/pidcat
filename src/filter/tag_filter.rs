use crate::filter::Filter;
use crate::log::Log;
use anyhow::Result;
use async_trait::async_trait;
use regex::{Error, Regex, RegexBuilder};
use std::sync::Arc;

pub(crate) struct TagFilter {
    filter: Option<Arc<dyn Filter>>,
    tag: String,
    re: Result<Regex, Error>,
}

impl TagFilter {
    pub fn new(tag: String, ignore: bool, filter: Option<Arc<dyn Filter>>) -> Self {
        Self {
            filter,
            tag: tag.clone(),
            re: RegexBuilder::new(&tag).case_insensitive(ignore).build(),
        }
    }
}

#[async_trait]
impl Filter for TagFilter {
    async fn filter(&self, mut log: Log) -> Option<Log> {
        if let Some(f) = &self.filter {
            let f = Arc::clone(f);
            if let Some(r) = f.filter(log).await {
                log = r;
            } else {
                return None;
            }
        }

        if self.tag.is_empty() {
            return Some(log);
        }

        if let Ok(re) = &self.re {
            if re.is_match(&log.tag) || re.is_match(&log.message) {
                Some(log)
            } else {
                None
            }
        } else {
            Some(log)
        }
    }
}
