use crate::log::Log;
use crate::sink::Sink;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::task;

pub struct FileSink {
    file: File,
}

impl FileSink {
    #[allow(dead_code)]
    pub async fn new(file: PathBuf) -> Result<Self> {
        let f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file)
            .await?;
        Ok(Self { file: f })
    }
}

#[async_trait]
impl Sink for FileSink {
    async fn write(&self, log: Log) {
        let s = format!(
            "{} {:11} {:>5} {:<5} {} {}   {}\n",
            log.date, log.time, log.pid, log.tid, log.level, log.tag, log.message
        );
        let file = self.file.try_clone().await;
        task::spawn(async move {
            if let Ok(mut file) = file {
                if let Ok(_) = file.write_all(s.as_bytes()).await {}
            }
        });
    }
}
