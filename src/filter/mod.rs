mod buffer_filter;
mod level_filter;
mod pid_filter;
mod revert_filter;
mod tag_filter;

#[allow(unused_imports)]
pub(crate) use buffer_filter::BufferFilter;
#[allow(unused_imports)]
pub(crate) use level_filter::LevelFilter;
#[allow(unused_imports)]
pub(crate) use pid_filter::PidFilter;
#[allow(unused_imports)]
pub(crate) use revert_filter::RevertFilter;
#[allow(unused_imports)]
pub(crate) use tag_filter::TagFilter;

use crate::log::Log;

use async_trait::async_trait;
use std::sync::Arc;

#[allow(dead_code)]
pub(crate) type ArcFilter = Arc<dyn Filter>;

///
/// Filter trait used to filter log
///
#[async_trait]
pub trait Filter: Send + Sync {
    async fn filter(&self, log: Log) -> Option<Log>;
}
