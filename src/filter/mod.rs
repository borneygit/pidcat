mod buffer_filter;
mod level_filter;
mod pid_filter;
mod revert_filter;
mod tag_filter;

pub(crate) use buffer_filter::BufferFilter;
pub(crate) use level_filter::LevelFilter;
pub(crate) use pid_filter::PidFilter;
pub(crate) use revert_filter::RevertFilter;
pub(crate) use tag_filter::TagFilter;
use crate::log::Log;

use std::sync::Arc;
use async_trait::async_trait;

pub(crate) type ArcFilter = Arc<dyn Filter>;

#[async_trait]
pub(crate) trait Filter: Send + Sync {
    async fn filter(&self, log: Log) -> Option<Log>;
}
