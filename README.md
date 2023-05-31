# pidcat

During Android application development, we often want to display only the current log information of our own application. Unfortunately, since the process ID changes every time it is deployed to the phone, isn't it trying to find a log filtering tool that can solve this problem.

pidcat is all about filtering the application logs by matching the application packages. Then you can enjoy a more convenient development process.

here is an example.

```
pidcat toor
```

![ScreenShot](/asset/screen.png)

# Install terminal
First install the rust environment
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then use cargo to install pidcat
```
cargo install --path .
```

# How to use terminal

```rust
pidcat --help

A logcat colored command which displays only source entries for processes of a specific application package.

Usage: pidcat [OPTIONS] [process]...

Arguments:
  [process]...
          Name of the process to be filtered

Options:
  -t, --tag <tag>
          The tag filter patterns

      --tag-width <tag_width>
          Set the tag show width. must >= 10
          [default: 20]

  -v, --revert-match <revert>
          Selected lines are those not matching any of the specified patterns.

  -b, --buffer <buffer>
          The buffer to filter
          [default: main system]
          [possible values: main, system, crash, radio, events, all]

  -c, --clear
          Clear (flush) the entire log and exit

  -l, --level <level>
          Filter log level
          [default: V]
          Possible values:
          - V: Verbose
          - D: Debug
          - I: Info
          - W: Warning
          - E: Error
          - F: Fatal

  -o, --output <output>
          Writing logs to a file

      --color <color>
          Display in highlighted color to match priority
          [default: auto]
          [possible values: auto, always, never]

  -i, --ignore-case
          Ignore case

  -s <device>
          Use device with given serial

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# Use by crate
add dep for Cargo.toml
```
[dependencies]
pidcat="0.2.0"
```
You can use the following code to capture the adb logcat logs and process them twice according to your needs
```rust
use futures::StreamExt;
use pidcat::LogStream;
use pidcat::source::*;

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
```

# Thanks
https://github.com/JakeWharton/pidcat </br>
https://github.com/flxo/rogcat
