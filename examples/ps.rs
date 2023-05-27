use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};

#[tokio::main]
async fn main() {
    let mut ps = spawn_ps().await;
    let pids = Vec::from([
        "launcher",
        "com.android.phone",
        "genymotion"
    ]);

    let reader_task = tokio::spawn(async move {
        let mut reader = tokio::io::BufReader::new(ps.stdout.take().unwrap());
        let mut line = String::new();

        while let Ok(bytes_read) = reader.read_line(&mut line).await {
            if bytes_read == 0 {
                break;
            }
            let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
            let name = spl[8];
            let pid= spl[1];
            for p in pids.iter() {
                if name.contains(p) {
                    println!("{}:{}", name, pid);
                }
            }
            line.clear();
        }
    });

    let _ = reader_task.await;
}

async fn spawn_ps() -> Child {
    let mut command = Command::new("adb");
    command.stdout(std::process::Stdio::piped());
    command.arg("shell");
    command.arg("ps");
    command.spawn()
        .expect("Failed to execute adb shell ps")
}
