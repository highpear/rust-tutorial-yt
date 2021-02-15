use std::process::Command;

fn main() {

    let mut cmd = Command::new("python3");  // aliasが無効になる？

    cmd.arg("hello.py");

    match cmd.output() {
        Ok(o) => {
            println!("Output: {}", String::from_utf8_lossy(&o.stdout));
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}