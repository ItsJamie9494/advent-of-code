use std::fmt::Display;
use std::process::{Command, Output, Stdio};

/// Wrapper module around the `aoc-cli` utility.
/// Currently, only supports downloading input files.

#[derive(Debug)]
pub enum CommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExit(Output),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::CommandNotFound => write!(f, "aoc-cli is not present in this environment. Please follow the directions at https://github.com/scarvalhojr/aoc-cli/"),
            CommandError::CommandNotCallable => write!(f, "aoc-cli could not be called. Please check that aoc-cli is executable"),
            CommandError::BadExit(output) => write!(f, "aoc-cli exited with a non-zero status:\n\n==STDOUT==\n{:?}\n\n==STDERR==\n{:?}", output.stdout, output.stderr),
        }
    }
}

fn check() -> Result<(), CommandError> {
    Command::new("aoc").arg("-V").output().map_err(|_| CommandError::CommandNotFound)?;
    Ok(())
}

pub fn download(day: &str) -> Result<Output, CommandError> {
    let input_path = format!("input/{day}.txt");

    let args = build_args(
        "download",
        &[
            "--overwrite".into(),
            "--input-only".into(),
            "--input-file".into(),
            input_path.into()
        ],
        day,
    );

    let output = call_cli(&args)?;
    Ok(output)
}

fn build_args(command: &str, args: &[String], day: &str) -> Vec<String> {
    let mut cmd_args = args.to_vec();

    if crate::get_year() != 0000 {
        cmd_args.push("--year".into());
        cmd_args.push(crate::get_year().to_string());
    }

    cmd_args.append(&mut vec!["--day".into(), day.to_string(), command.into()]);

    cmd_args
}

fn call_cli(args: &[String]) -> Result<Output, CommandError> {
    check()?;

    let output = Command::new("aoc")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| CommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(CommandError::BadExit(output))
    }
}