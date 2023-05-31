use crate::filter::Filter;
use crate::log::Log;
use async_trait::async_trait;
use regex::{Error, Regex, RegexBuilder};
use std::sync::Arc;

pub(crate) struct RevertFilter {
    filter: Option<Arc<dyn Filter>>,
    revert: String,
    re: Result<Regex, Error>,
}

impl RevertFilter {
    #[allow(dead_code)]
    pub(crate) fn new(revert: String, ignore: bool, filter: Option<Arc<dyn Filter>>) -> Self {
        Self {
            filter,
            revert: revert.clone(),
            re: RegexBuilder::new(&revert).case_insensitive(ignore).build(),
        }
    }
}

#[async_trait]
impl Filter for RevertFilter {
    async fn filter(&self, mut log: Log) -> Option<Log> {
        if let Some(f) = &self.filter {
            let f = Arc::clone(f);
            if let Some(r) = f.filter(log).await {
                log = r;
            } else {
                return None;
            }
        }

        if self.revert.is_empty() {
            return Some(log);
        }

        if let Ok(re) = &self.re {
            if re.is_match(&log.tag) || re.is_match(&log.message) {
                None
            } else {
                Some(log)
            }
        } else {
            Some(log)
        }
    }
}
