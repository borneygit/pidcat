use std::fmt::{Display, Formatter};
use std::pin::Pin;
use futures::{Sink, Stream, StreamExt};
use anyhow::Result;
use async_stream::stream;
use std::collections::HashMap;
use async_trait::async_trait;
use regex::Regex;
use tokio::io::AsyncBufReadExt;
use std::error::Error;
use tokio::process::{Child, Command};

#[derive(Debug, Clone)]
pub(crate) struct Log {
    pub date: String,
    pub time: String,
    pub pid: String,
    pub tid: String,
    pub level: String,
    pub tag: String,
    pub message: String,
    pub buffer: String,
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {} {} {} {}", self.date, self.time, self.pid, self.tid, self.level, self.tag, self.message)
    }
}

type LogStream = Pin<Box<dyn Stream<Item=Result<Log, Box<dyn Error + Send>>> + Send>>;

#[async_trait]
trait Source {
    async fn source(&self) -> LogStream;
}

struct AdbSource;

impl AdbSource {
    async fn spawn_adb_logcat(&self) -> Child {
        let mut command = Command::new("adb");
        command.stdout(std::process::Stdio::piped());
        command.arg("logcat");
        command.arg("-D");
        command.arg("-v").arg("long");
        command.arg("-b").arg("all");
        command.spawn()
            .expect("Failed to execute adb logcat")
    }
}

#[async_trait]
impl Source for AdbSource {
    async fn source(&self) -> LogStream {
        let mut logcat = self.spawn_adb_logcat().await;
        let mut reader = tokio::io::BufReader::new(logcat.stdout.take().unwrap());
        let re = Regex::new(r"\[ (\d{2}-\d{2})\s(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\d+):\s+(\d+)\s+(.*) ]").unwrap();

        let s = stream! {
            let mut line = String::new();
            let mut buffer = String::new();
            let mut map = HashMap::new();

            while let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }

                if line.starts_with("---------") {
                    let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
                    buffer = spl[3].to_string();
                    map.insert("buffer", buffer.clone());
                } else if line.trim().is_empty() {
                    if map.contains_key("message") {
                        let log = Log {
                            tag: map.remove("tag").unwrap(),
                            date: map.remove("date").unwrap(),
                            time: map.remove("time").unwrap(),
                            pid: map.remove("pid").unwrap(),
                            tid: map.remove("tid").unwrap(),
                            level: map.remove("level").unwrap(),
                            message: map.remove("message").unwrap(),
                            buffer: map.get("buffer").unwrap().clone(),
                        };
                        yield Ok(log);
                    }
                } else {
                    if let Some(captures) = re.captures(&line) {
                        map.insert("date", captures.get(1).unwrap().as_str().to_string());
                        map.insert("time", captures.get(2).unwrap().as_str().to_string());
                        map.insert("pid", captures.get(3).unwrap().as_str().to_string());
                        map.insert("tid", captures.get(4).unwrap().as_str().to_string());
                        let content = captures.get(5).unwrap().as_str().to_string();
                        let content = content.splitn(2, '/').collect::<Vec<&str>>();
                        map.insert("level", content[0].to_string());
                        map.insert("tag", content[1].to_string());
                    } else {
                        if let Some(msg) = map.get_mut("message") {
                            msg.push('\n');
                            msg.push_str(line.trim_end());
                        } else {
                            map.insert("message", line.trim_end().to_string());
                        }
                    }
                }
                line.clear();
            }
        };
        return Box::pin(s);
    }
}

#[tokio::main]
async fn main() {
    let adb = AdbSource;

    tokio::spawn(async move {
        let mut logs = adb.source().await;
        while let Some(r) = logs.next().await {
            match r {
                Ok(log) => {
                    println!("{}", log);
                }
                Err(_) => {}
            }
        }
    }).await.expect("TODO: panic message");

}