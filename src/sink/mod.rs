mod file;
mod terminal;

use crate::log::Log;
use async_trait::async_trait;

#[allow(unused_imports)]
pub(crate) use file::FileSink;
#[allow(unused_imports)]
pub(crate) use terminal::TerminalSink;

///
/// Sink trait used to develop log out sink
///
#[async_trait]
pub trait Sink: Send + Sync {
    async fn write(&self, log: Log);
}
