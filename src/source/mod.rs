use crate::log::{Log, LogStream};
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Source: Send + Sync {
    async fn source(&self) -> LogStream;
}

mod adb;

pub(crate) use adb::ADBSource;

mod file;
