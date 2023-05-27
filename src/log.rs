use anyhow::Result;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use futures::Stream;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::pin::Pin;
use std::str::FromStr;

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

pub(crate) type LogStream = Pin<Box<dyn Stream<Item = Result<Log, Box<dyn Error + Send>>> + Send>>;

#[allow(dead_code)]
impl Log {
    pub fn is_main(&self) -> bool {
        return self.buffer == "main";
    }

    pub fn is_system(&self) -> bool {
        return self.buffer == "system";
    }

    pub fn is_crash(&self) -> bool {
        return self.buffer == "crash";
    }

    pub fn is_events(&self) -> bool {
        return self.buffer == "events";
    }

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
