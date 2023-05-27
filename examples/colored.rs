use colored::*;
use lazy_static::lazy_static;

struct Color(u8, u8, u8);

lazy_static! {
    static ref ERROR: Color = Color(
        u8::from_str_radix(&"#ff2600"[1..3], 16).unwrap(),
        u8::from_str_radix(&"#ff2600"[3..5], 16).unwrap(),
        u8::from_str_radix(&"#ff2600"[5..7], 16).unwrap()
    );

    static ref VERBOSE: Color = Color(
        u8::from_str_radix(&"#ffffff"[1..3], 16).unwrap(),
        u8::from_str_radix(&"#ffffff"[3..5], 16).unwrap(),
        u8::from_str_radix(&"#ffffff"[5..7], 16).unwrap()
    );

    static ref INFO: Color = Color(
        u8::from_str_radix(&"#05d702"[1..3], 16).unwrap(),
        u8::from_str_radix(&"#05d702"[3..5], 16).unwrap(),
        u8::from_str_radix(&"#05d702"[5..7], 16).unwrap()
    );

    static ref WARNING: Color = Color(
        u8::from_str_radix(&"#d75f02"[1..3], 16).unwrap(),
        u8::from_str_radix(&"#d75f02"[3..5], 16).unwrap(),
        u8::from_str_radix(&"#d75f02"[5..7], 16).unwrap()
    );

    static ref DEBUG: Color = Color(
        u8::from_str_radix(&"#5fafff"[1..3], 16).unwrap(),
        u8::from_str_radix(&"#5fafff"[3..5], 16).unwrap(),
        u8::from_str_radix(&"#5fafff"[5..7], 16).unwrap()
    );
}

macro_rules! error {
    ($($tt:tt)*) => {
        println!("{}", format!($($tt)*).truecolor(ERROR.0, ERROR.1, ERROR.2));
    }
}

macro_rules! warn {
    ($($tt:tt)*) => {
        println!("{}", format!($($tt)*).truecolor(WARNING.0, WARNING.1, WARNING.2));
    }
}

macro_rules! info {
    ($($tt:tt)*) => {
        println!("{}", format!($($tt)*).on_truecolor(INFO.0, INFO.1, INFO.2));
    }
}

macro_rules! debug {
    ($($tt:tt)*) => {
        println!("{}", format!($($tt)*).on_truecolor(DEBUG.0, DEBUG.1, DEBUG.2));
    }
}

macro_rules! verbose {
    ($($tt:tt)*) => {
        println!("{}", format!($($tt)*).truecolor(VERBOSE.0, VERBOSE.1, VERBOSE.2));
    }
}

fn main() {
    error!("hello {} {}", 123, "abc");
    info!("hello {} {}", 123, "abc");
    debug!("hello {} {}", 123, "abc");
    warn!("hello {} {}", 123, "abc");
    verbose!("hello {} {}", 123, "abc");
}