mod file;
mod terminal;

use crate::log::Log;
use async_trait::async_trait;

pub(crate) use file::FileSink;
pub(crate) use terminal::TerminalSink;

#[async_trait]
pub(crate) trait Sink: Send + Sync {
    async fn write(&self, log: Log);
}
