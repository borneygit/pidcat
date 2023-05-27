use super::*;
use async_stream::stream;
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};

pub(crate) struct ADBSource {
    device: String,
}

impl ADBSource {
    pub fn new(device: String) -> Self {
        Self { device }
    }

    async fn spawn_adb_logcat(&self) -> Child {
        let mut command = Command::new("adb");
        command.stdout(std::process::Stdio::piped());
        if !self.device.is_empty() {
            command.arg("-s").arg(&self.device);
        }
        command.arg("logcat");
        command.arg("-D");
        command.arg("-v").arg("long");
        command.arg("-b").arg("all");
        command.spawn().expect("Failed to execute adb logcat")
    }
}

#[async_trait]
impl Source for ADBSource {
    async fn source(&self) -> LogStream {
        let mut logcat = self.spawn_adb_logcat().await;
        let mut reader = tokio::io::BufReader::new(logcat.stdout.take().unwrap());

        let s = stream! {
            let mut line = String::new();
            let mut buffer = String::new();
            let mut map = HashMap::new();

            let re = Regex::new(r"\[ (\d{2}-\d{2})\s(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\d+):(.*) ]").unwrap();

            while let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                if line.starts_with("---------") {
                    if map.contains_key("tag") {
                        let log = Log {
                            tag: map.remove("tag").unwrap(),
                            date: map.remove("date").unwrap(),
                            time: map.remove("time").unwrap(),
                            pid: map.remove("pid").unwrap(),
                            tid: map.remove("tid").unwrap(),
                            level: map.remove("level").unwrap(),
                            message: map.remove("message").unwrap().trim_end().to_string(),
                            buffer: map.get("buffer").unwrap().clone(),
                        };
                        yield Ok(log);
                    }

                    let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
                    buffer = spl[3].to_string();
                    map.insert("buffer", buffer.clone());
                } else {
                    if let Some(cap) = re.captures(&line) {
                        if map.contains_key("tag") {
                            let log = Log {
                                tag: map.remove("tag").unwrap(),
                                date: map.remove("date").unwrap(),
                                time: map.remove("time").unwrap(),
                                pid: map.remove("pid").unwrap(),
                                tid: map.remove("tid").unwrap(),
                                level: map.remove("level").unwrap(),
                                message: map.remove("message").unwrap().trim_end().to_string(),
                                buffer: map.get("buffer").unwrap().clone(),
                            };
                            yield Ok(log);
                        }

                        map.insert("date", cap.get(1).unwrap().as_str().to_string());
                        map.insert("time", cap.get(2).unwrap().as_str().to_string());
                        map.insert("pid", cap.get(3).unwrap().as_str().to_string());
                        let content = cap.get(4).unwrap().as_str().trim_start().to_string();
                        let content = content.splitn(2, ' ').collect::<Vec<&str>>();
                        map.insert("tid", content[0].trim().to_string());
                        let content = content[1].to_string();
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
