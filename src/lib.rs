use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::*;

type PartFunction = &'static dyn Fn() -> Result<()>;

pub fn start_day(day: &str, part1: PartFunction, part2: PartFunction) -> Result<()> {
    println!("Advent of Code 2024 - Day {day}");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a part argument");
    }

    let part_to_run = args[1].parse::<i32>().unwrap_or(1);

    if part_to_run < 1 || part_to_run > 2 {
        panic!("Invalid argument");
    }

    println!("==== PART {} ====", part_to_run);
    if part_to_run == 1 {
        part1()?;
    } else if part_to_run == 2 {
        part2()?;
    }

    Ok(())
}

fn get_input_file(day: &str) -> Result<BufReader<File>> {
    let filename = format!("input/{day}.txt");
    println!("Reading file {filename}");
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

pub fn input_as_lines(day: &str) -> Result<Vec<String>> {
    let file = get_input_file(day);
    let lines: Vec<String> = file?.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}