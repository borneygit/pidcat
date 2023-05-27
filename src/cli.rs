use crate::log::Level;
use anyhow::Result;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct Cli {
    pub tag: String,
    pub tag_width: usize,
    pub revert: String,
    pub clear: bool,
    pub level: Level,
    pub color: String,
    pub output: Option<PathBuf>,
    pub process: Vec<String>,
    pub buffers: Vec<String>,
    pub device: String,
    pub ignore: bool,
}

pub(crate) fn cli() -> Result<Cli> {
    let matches = get_arg_matches();
    let tag = match matches.get_one::<String>("tag") {
        None => "",
        Some(s) => s,
    }
    .to_owned();

    let revert = match matches.get_one::<String>("revert") {
        None => "",
        Some(s) => s,
    }
    .to_owned();

    let tag_width = match matches.get_one::<String>("tag_width") {
        None => "20",
        Some(s) => s,
    }
    .parse::<usize>()
    .unwrap_or(20);

    let output = matches.get_one::<PathBuf>("output");
    let color = matches.get_one::<String>("color").unwrap().to_owned();
    let process = get_many(&matches, "process");
    let buffers = get_many(&matches, "buffer");
    let clear = matches.get_flag("clear");
    let level = matches.get_one::<Level>("level").unwrap().to_owned();
    let ignore = matches.get_flag("ignore");
    let device = match matches.get_one::<String>("device") {
        None => "",
        Some(s) => s,
    }
    .to_owned();

    Ok(Cli {
        tag,
        tag_width,
        revert,
        color,
        clear,
        output: output.cloned(),
        level,
        process,
        buffers,
        device,
        ignore,
    })
}

fn get_arg_matches() -> ArgMatches {
    let crate_name: &str = env!("CARGO_PKG_NAME");
    let version: &str = env!("CARGO_PKG_VERSION");
    let author: &str = env!("CARGO_PKG_AUTHORS");
    Command::new(crate_name)
        .version(version)
        .author(author)
        .about("A logcat colored command which displays only source entries for processes of a specific application package.")
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .help("The tag filter patterns")
                .conflicts_with("revert")
        )
        .arg(
            Arg::new("tag_width")
                .long("tag-width")
                .default_value("20")
                .help("Set the tag show width. must >= 10")
        )
        .arg(
            Arg::new("revert")
                .short('v')
                .long("revert-match")
                .help("Selected lines are those not matching any of the specified patterns.")
                .conflicts_with("tag")
        )
        .arg(
            Arg::new("buffer")
                .short('b')
                .default_values(["main", "system"])
                .long("buffer")
                .value_parser(["main", "system", "crash", "radio", "events", "all"])
                .help("The buffer to filter")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("clear")
                .short('c')
                .long("clear")
                .help("Clear (flush) the entire log and exit")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("level")
                .short('l')
                .long("level")
                .help("Filter log level")
                .default_value("V")
                .value_parser(value_parser!(Level))
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(false)
                .help("Writing logs to a file")
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            Arg::new("color")
                .long("color")
                .help("Display in highlighted color to match priority")
                .default_value("auto")
                .value_parser(["auto", "always", "never"])
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore-case")
                .help("Ignore case")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("device")
                .required(false)
                .short('s')
                .help("Use device with given serial")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("process")
                .help("Name of the process to be filtered")
                .action(ArgAction::Append)
        )
        .get_matches()
}

fn get_many<'a>(matches: &ArgMatches, arg: &str) -> Vec<String> {
    matches
        .get_many::<String>(arg)
        .unwrap_or_default()
        .into_iter()
        .map(|v| v.to_owned())
        .collect::<Vec<String>>()
}
