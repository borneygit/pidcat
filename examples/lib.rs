use futures::StreamExt;
use pidcat::source::*;
use pidcat::LogStream;

#[tokio::main]
async fn main() {
    let source = ADBSource::new(None);

    let mut logs: LogStream = source.source().await;

    while let Some(r) = logs.next().await {
        if let Ok(log) = r {
            println!("{}", log);
        }
    }
}
