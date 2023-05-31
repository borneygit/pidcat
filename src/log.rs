use anyhow::Result;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use futures::Stream;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::str::FromStr;

///
/// Wrapping of Adb logs.
///
#[derive(Debug, Clone)]
pub struct Log {
    /// Log date
    pub date: String,
    /// Log time
    pub time: String,
    /// Log pid
    pub pid: String,
    /// Log tid
    pub tid: String,
    /// Log level
    pub level: String,
    /// Log tag
    pub tag: String,
    /// Log message, stored in branches, split with '\n'
    pub message: String,
    /// Log buffer, contains  'main', 'system', 'radio', 'events', 'crash'
    pub buffer: String,
}

///
/// Log out Stream
///
/// #Examples
/// ```no_run
/// let source: = ADBSource::new(None);
/// let mut logs = source.source().await;
/// while let Some(r) = logs.next().await {
///     match r {
///         Ok(log) => {
///             println!("{}", log);
///         }
///         Err(_) => {}
///     }
/// }
/// ```
///
pub type LogStream = Pin<Box<dyn Stream<Item = Result<Log, Box<dyn Error + Send>>> + Send>>;

#[allow(dead_code)]
impl Log {
    /// log is main buffer
    pub fn is_main(&self) -> bool {
        return self.buffer == "main";
    }

    /// log is system buffer
    pub fn is_system(&self) -> bool {
        return self.buffer == "system";
    }

    /// log is crash buffer
    pub fn is_crash(&self) -> bool {
        return self.buffer == "crash";
    }

    /// log is events buffer
    pub fn is_events(&self) -> bool {
        return self.buffer == "events";
    }

    /// log is radio buffer
    pub fn is_radio(&self) -> bool {
        return self.buffer == "radio";
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.date, self.time, self.pid, self.tid, self.level, self.tag, self.message
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum Level {
    V,
    D,
    I,
    W,
    E,
    F,
    S,
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "V" => Ok(Level::V),
            "D" => Ok(Level::D),
            "I" => Ok(Level::I),
            "W" => Ok(Level::W),
            "E" => Ok(Level::E),
            "F" => Ok(Level::F),
            _ => Ok(Level::S),
        }
    }
}

impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        match value {
            Level::V => 1,
            Level::D => 2,
            Level::I => 3,
            Level::W => 4,
            Level::E => 5,
            Level::F => 6,
            Level::S => 7,
        }
    }
}

impl ValueEnum for Level {
    fn value_variants<'a>() -> &'a [Self] {
        &[Level::V, Level::D, Level::I, Level::W, Level::E, Level::F]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Level::V => PossibleValue::new("V").help("Verbose"),
            Level::D => PossibleValue::new("D").help("Debug"),
            Level::I => PossibleValue::new("I").help("Info"),
            Level::W => PossibleValue::new("W").help("Warning"),
            Level::E => PossibleValue::new("E").help("Error"),
            Level::F => PossibleValue::new("F").help("Fatal"),
            Level::S => PossibleValue::new("S").help("Silent"),
        })
    }
}
