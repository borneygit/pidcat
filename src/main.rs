use crate::cli::Cli;
use crate::filter::{BufferFilter, Filter, LevelFilter, PidFilter, RevertFilter, TagFilter};
use crate::sink::{FileSink, Sink, TerminalSink};
use crate::source::{ADBSource, Source};
use anyhow::Result;
use futures::StreamExt;
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
    let source = ADBSource::new(if cli.device.is_empty() {
        None
    } else {
        Some(cli.device)
    });

    let filters: Vec<Box<dyn Filter>> = vec![
        Box::new(PidFilter::new(cli.process)),
        Box::new(BufferFilter::new(cli.buffers)),
        Box::new(LevelFilter::new(cli.level)),
        Box::new(TagFilter::new(cli.tag, cli.ignore)),
        Box::new(RevertFilter::new(cli.revert, cli.ignore)),
    ];

    let mut sinks: Vec<Box<dyn Sink>> = Vec::new();

    sinks.push(Box::new(TerminalSink::new(cli.color, cli.tag_width)));
    if let Some(file) = cli.output {
        if let Ok(file) = FileSink::new(file).await {
            sinks.push(Box::new(file));
        }
    }

    let mut logs = source.source().await;

    while let Some(r) = logs.next().await {
        match r {
            Ok(log) => {
                let mut is_filter = false;
                for filter in &filters {
                    if filter.filter(&log).await {
                        is_filter = true;
                        break;
                    }
                }

                if !is_filter {
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
