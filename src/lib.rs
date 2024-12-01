mod aoc_helper;

use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::result::Result::Ok;

use aoc_helper::download;

type PartFunction = &'static dyn Fn() -> Result<()>;

pub fn start_day(day: &str, part1: PartFunction, part2: PartFunction) -> Result<()> {
    println!("Advent of Code {:?} - Day {}", get_year(), day);

    let args: Vec<String> = std::env::args().collect();
    let part_to_run: i32;
    if args.len() < 2 {
        part_to_run = 0;
    } else {
        part_to_run = args[1].parse::<i32>().unwrap_or(1);
    }

    if part_to_run < 0 || part_to_run > 2 {
        panic!("Invalid argument");
    }

    if part_to_run == 1 {
        println!("==== PART 1 ====");
        part1()?;
    } else if part_to_run == 2 {
        println!("==== PART 2 ====");
        part2()?;
    } else if part_to_run == 0 {
        println!("==== PART 1 ====");
        part1()?;
        println!("==== PART 2 ====");
        part2()?;
    }

    Ok(())
}

fn get_year() -> u16 {
    match std::env::var("AOC_YEAR") {
        Ok(x) => x.parse::<u16>().unwrap_or(0000),
        Err(_) => 0000,
    }
}

fn get_input_file(day: &str) -> Result<BufReader<File>> {
    let filename = format!("input/{day}.txt");
    println!("Reading file {filename}");
    // We don't recreate the input file unless it's missing
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(_) => {
            if let Err(e) = download(day) {
                eprintln!("aoc cli failed: {e}");
                process::exit(1);
            }
            // safety: This could result in an infinite loop. I guess that's fine, we can manually kill it.
            Ok(get_input_file(day)?)
        }
    }
}

pub fn input_as_lines(day: &str) -> Result<Vec<String>> {
    let file = get_input_file(day);
    let lines: Vec<String> = file?.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}