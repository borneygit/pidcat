use crate::filter::Filter;
use crate::log::Log;
use anyhow::Result;
use async_trait::async_trait;
use regex::{Error, Regex, RegexBuilder};

pub(crate) struct TagFilter {
    tag: String,
    re: Result<Regex, Error>,
}

impl TagFilter {
    #[allow(dead_code)]
    pub(crate) fn new(tag: String, ignore: bool) -> Self {
        Self {
            tag: tag.clone(),
            re: RegexBuilder::new(&tag).case_insensitive(ignore).build(),
        }
    }
}

#[async_trait]
impl Filter for TagFilter {
    async fn filter(&self, log: &Log) -> bool {
        if self.tag.is_empty() {
            return false;
        }

        if let Ok(re) = &self.re {
            if re.is_match(&log.tag) || re.is_match(&log.message) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}
