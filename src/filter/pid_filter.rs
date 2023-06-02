use crate::filter::Filter;
use crate::log::Log;
use async_trait::async_trait;
use dashmap::DashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};

pub(crate) struct PidFilter {
    process: DashSet<String>,
    pids: DashSet<String>,
    first_filter: AtomicBool,
}

impl PidFilter {
    #[allow(dead_code)]
    pub(crate) fn new(process: Vec<String>) -> Self {
        Self {
            process: DashSet::from_iter(process.into_iter()),
            pids: DashSet::new(),
            first_filter: AtomicBool::new(true),
        }
    }

    async fn spawn_ps(&self) -> Child {
        let mut command = Command::new("adb");
        command.stdout(std::process::Stdio::piped());
        command.arg("shell");
        command.arg("ps");
        command.spawn().expect("Failed to execute adb shell ps")
    }
}

#[async_trait]
impl Filter for PidFilter {
    async fn filter(&self, log: &Log) -> bool {
        if self.process.is_empty() {
            return false;
        }

        if self.first_filter.load(Ordering::Acquire) && self.pids.is_empty() {
            let mut ps = self.spawn_ps().await;
            let mut reader = tokio::io::BufReader::new(ps.stdout.take().unwrap());
            let mut line = String::new();

            while let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
                let name = spl[8];
                let pid = spl[1];
                for p in self.process.iter() {
                    let ptr = p.key().as_str();
                    if name.contains(ptr) {
                        self.pids.insert(pid.to_string());
                    }
                }
                line.clear();
            }
            self.first_filter.store(false, Ordering::Release);
        }

        let mut remove_pid = None;

        let message = &log.message;

        match log.tag.as_ref() {
            "am_proc_start" => {
                let spl = message[1..message.len() - 1]
                    .split(",")
                    .collect::<Vec<&str>>();
                let pid = spl[1];
                let name = spl[3];
                for p in self.process.iter() {
                    let ptr = p.key().as_str();
                    if name.contains(ptr) {
                        self.pids.insert(pid.to_string());
                    }
                }
            }
            "am_proc_died" => {
                let spl = message[1..message.len() - 1]
                    .split(",")
                    .collect::<Vec<&str>>();
                let pid = spl[1];
                let _name = spl[2];
                remove_pid = Some(pid.to_string());
            }
            _ => {}
        }

        let mut r = true;

        if self.pids.contains(&log.pid) {
            r = false;
        } else {
            for p in self.pids.iter() {
                let ptr = p.key().as_str();
                if log.is_events() && log.message.contains(ptr) {
                    r = false;
                    break;
                }
            }
        }

        if let Some(pid) = remove_pid {
            self.pids.remove(pid.as_str());
        }
        return r;
    }
}
