use crate::filter::Filter;
use crate::log::Log;
use async_trait::async_trait;
use regex::{Error, Regex, RegexBuilder};

pub struct RevertFilter {
    revert: String,
    re: Result<Regex, Error>,
}

impl RevertFilter {
    #[allow(dead_code)]
    pub fn new(revert: String, ignore: bool) -> Self {
        Self {
            revert: revert.clone(),
            re: RegexBuilder::new(&revert).case_insensitive(ignore).build(),
        }
    }
}

#[async_trait]
impl Filter for RevertFilter {
    async fn filter(&self, log: &Log) -> bool {
        if self.revert.is_empty() {
            return false;
        }

        if let Ok(re) = &self.re {
            if re.is_match(&log.tag) || re.is_match(&log.message) {
                return true;
            }
        }
        false
    }
}
