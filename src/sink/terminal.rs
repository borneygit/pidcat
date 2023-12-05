use crate::log::Log;
use crate::sink::Sink;
use async_trait::async_trait;
use colored::*;
use lazy_static::lazy_static;

struct Color(u8, u8, u8);

lazy_static! {
    static ref ERROR: Color = Color(255, 38, 0);
    static ref VERBOSE: Color = Color(255, 255, 255);
    static ref INFO: Color = Color(5, 215, 2);
    static ref WARNING: Color = Color(215, 95, 2);
    static ref DEBUG: Color = Color(95, 175, 255);
}

macro_rules! error {
    ($s:expr) => {
        format!("{}", $s.truecolor(ERROR.0, ERROR.1, ERROR.2))
    };
}

macro_rules! warn {
    ($s:expr) => {
        format!("{}", $s.truecolor(WARNING.0, WARNING.1, WARNING.2))
    };
}

macro_rules! info {
    ($s:expr) => {
        format!("{}", $s.truecolor(INFO.0, INFO.1, INFO.2))
    };
}

macro_rules! debug {
    ($s:expr) => {
        format!("{}", $s.truecolor(DEBUG.0, DEBUG.1, DEBUG.2))
    };
}

macro_rules! verbose {
    ($s:expr) => {
        format!("{}", $s.truecolor(VERBOSE.0, VERBOSE.1, VERBOSE.2))
    };
}

pub struct TerminalSink {
    color: bool,
    tag_width: usize,
}

impl TerminalSink {
    #[allow(dead_code)]
    pub fn new(color: String, tag_width: usize) -> Self {
        Self {
            color: color == "always" || color == "auto",
            tag_width,
        }
    }

    fn format_by_level(&self, level: &str, s: &str) -> String {
        if self.color {
            match level {
                "V" => verbose!(s),
                "D" => debug!(s),
                "I" => info!(s),
                "W" => warn!(s),
                "E" => error!(s),
                "F" => error!(s),
                _ => verbose!(s),
            }
        } else {
            s.to_string()
        }
    }
}

#[async_trait]
impl Sink for TerminalSink {
    async fn write(&self, log: Log) {
        let mut tag = log.tag;
        if tag.len() > self.tag_width {
            tag.truncate(self.tag_width);
        }
        let message = &log.message.split('\n').collect::<Vec<&str>>();
        for (i, &s) in message.iter().enumerate() {
            let s = if i == 0 {
                let level = format!(" {} ", self.format_by_level(&log.level, &log.level))
                    .on_truecolor(88, 88, 88);
                let tag = self.format_by_level(
                    &log.level,
                    &format!("{:width$}", tag, width = self.tag_width),
                );
                format!(
                    "{:11} {:>5}-{:<5} {} {} {}",
                    log.time,
                    log.pid,
                    log.tid,
                    tag,
                    level,
                    self.format_by_level(&log.level, s),
                )
            } else {
                let level = log.level.on_truecolor(88, 88, 88);
                format!(
                    "{:12} {:>5}-{:<5} {:width$} {:^3} {}",
                    "",
                    "",
                    "",
                    " ",
                    level,
                    self.format_by_level(&log.level, s),
                    width = self.tag_width
                )
            };
            println!("{}", s);
        }
    }
}
