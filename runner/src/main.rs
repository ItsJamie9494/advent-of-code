// ADVENT OF CODE 2022
// COPYRIGHT (C) 2022 JAMIE MURPHY

use chrono::{Datelike, Utc};
use chrono_tz::EST;
use owo_colors::OwoColorize;
use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
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

    Command::new("cargo")
        .args(["run", "-p", &format!("day{}", current_project)])
        .exec();
}
