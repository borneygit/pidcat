use crate::cli::Cli;
use crate::filter::{ArcFilter, BufferFilter, LevelFilter, PidFilter, RevertFilter, TagFilter};
use crate::sink::{FileSink, Sink, TerminalSink};
use crate::source::{ADBSource, Source};
use anyhow::Result;
use futures::StreamExt;
use std::sync::Arc;
use tokio::process::{Child, Command};

mod cli;
mod filter;
mod log;
mod sink;
mod source;

async fn run() -> Result<()> {
    let cli = cli::cli()?;
    let _ = which::which("adb")?;

    tokio::spawn(async move {
        if cli.clear {
            spawn_adb_logcat_clear().await;
        } else {
            fetch(cli).await;
        }
    })
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(_) => {}
    }
}

async fn fetch(cli: Cli) {
    let source = Arc::new(ADBSource::new(if cli.device.is_empty() {
        None
    } else {
        Some(cli.device)
    }));

    let filter: ArcFilter = Arc::new(PidFilter::new(cli.process, None));
    let filter: ArcFilter = Arc::new(BufferFilter::new(cli.buffers, Some(filter)));
    let filter: ArcFilter = Arc::new(LevelFilter::new(cli.level, Some(filter)));
    let filter: ArcFilter = Arc::new(TagFilter::new(cli.tag, cli.ignore, Some(filter)));
    let filter: ArcFilter = Arc::new(RevertFilter::new(cli.revert, cli.ignore, Some(filter)));

    let mut sinks: Vec<Arc<dyn Sink>> = Vec::new();
    sinks.push(Arc::new(TerminalSink::new(cli.color, cli.tag_width)));
    if let Some(file) = cli.output {
        if let Ok(file) = FileSink::new(file).await {
            sinks.push(Arc::new(file));
        }
    }

    let mut logs = source.source().await;

    while let Some(r) = logs.next().await {
        match r {
            Ok(log) => {
                let l = { filter.filter(log).await };

                if let Some(log) = l {
                    for sink in &sinks {
                        sink.write(log.clone()).await;
                    }
                }
            }
            Err(_) => {}
        }
    }
}

async fn spawn_adb_logcat_clear() -> Child {
    let mut command = Command::new("adb");
    command.stdout(std::process::Stdio::piped());
    command.arg("logcat");
    command.arg("-c");
    command.spawn().expect("Failed to execute adb logcat -c")
}
