use crate::log::{Log, LogStream};
use async_trait::async_trait;

///
/// Source trait used to develop log input source
///
#[async_trait]
pub trait Source: Send + Sync {
    async fn source(&self) -> LogStream;
}

mod adb;

pub use adb::ADBSource;

mod file;
