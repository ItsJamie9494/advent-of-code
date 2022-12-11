// ADVENT OF CODE 2022
// COPYRIGHT (C) 2022 JAMIE MURPHY

use chrono::{Datelike, Utc};
use chrono_tz::EST;
use owo_colors::OwoColorize;
use std::{env, process::Command, time::Instant};

fn main() {
    let time = Instant::now();
    let divider = "===".bold();
    let current_day = Utc::now().with_timezone(&EST);
    println!("{}", "Advent of Code 2022".bold().underline());
    println!(
        "Current Day: {} ({} EST)",
        current_day.day().bold(),
        current_day.format("%d/%m/%Y %H:%M").italic()
    );

    let mut current_project = current_day.day();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        current_project = args[1].parse::<u32>().expect("Failed to parse project");
        println!("Running Project {}", current_project.bold());
    }

    let output = Command::new("cargo")
        .args(["run", "-p", &format!("day{}", current_project)])
        .output()
        .unwrap();

    let stdout: String = String::from_utf8(output.stdout).unwrap();
    if !stdout.is_empty() {
        println!(
            "\n{} {} {}\n{}",
            divider,
            "OUTPUT".bold().green(),
            divider,
            stdout
        );
    }
    let error: String = String::from_utf8(output.stderr).unwrap();
    if error.contains("thread") {
        println!(
            "\n{} {} {}\n{}",
            divider,
            "ERRORS".bold().red(),
            divider,
            error
        );
    }

    let elapsed = time.elapsed().as_secs_f64();

    println!("{}", format!("Took {:.2}s", elapsed).italic());
}
