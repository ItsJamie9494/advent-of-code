use std::process::{Command, Stdio};

pub fn run(day: String, part: i32) {
    let mut cmd_args = vec!["run".to_string(), "--release".to_string(), "--bin".to_string(), day];

    if part != 0 {
        cmd_args.push(part.to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}