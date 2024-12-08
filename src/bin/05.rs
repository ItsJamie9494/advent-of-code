use advent_of_code::*;
use anyhow::*;

const DAY: &str = "05";

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mut page_rules = vec![];
    let mut page = vec![];

    for line in lines {
        if line.contains("|") {
            page_rules.push(line.clone());
        }

        if line.contains(",") {
            page.push(line);
        }
    }

    let mut ret = 0;
    for line in page {
        let mut s = line.split(",");

        let mut valid = true;
        for char in s.clone() {
            for l in &page_rules {
                if let Some(_) = l.split("|").position(|x| x == char) {
                    let rule = l.split("|").collect::<Vec<&str>>();
                    let first = line.split(",").position(|x| x == rule[0]);
                    let second = line.split(",").position(|x| x == rule[1]);

                    if first.is_some() && second.is_some() {
                        if first.unwrap() > second.unwrap() {
                            valid = false;
                        }
                    }
                }
            }
        }

        if valid {
            let middle = ((s.clone().count() / 2) as f32).ceil() as usize;
            let nums: Vec<&str> = line.split(",").collect::<Vec<&str>>();
            ret += nums[middle].parse::<usize>()?;
        }
    }

    print!("Part 1: {ret}");

    Ok(())
}

fn valid_line(s: &Vec<&str>, page_rules: Vec<String>, line: String) -> bool {
    let mut valid = true;
    for char in s.clone() {
        for l in &page_rules {
            if let Some(_) = l.split("|").position(|x| x == char) {
                let rule = l.split("|").collect::<Vec<&str>>();
                let first = line.split(",").position(|x| x == rule[0]);
                let second = line.split(",").position(|x| x == rule[1]);

                if first.is_some() && second.is_some() {
                    if first.unwrap() > second.unwrap() {
                        valid = false;
                    }
                }
            }
        }
    }

    valid
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mut page_rules = vec![];
    let mut page = vec![];

    for line in lines {
        if line.contains("|") {
            page_rules.push(line.clone());
        }

        if line.contains(",") {
            page.push(line);
        }
    }

    let mut ret = 0;
    for line in page {
        let s = line.split(",");

        let valid = valid_line(&s.clone().collect::<Vec<&str>>(), page_rules.clone(), line.clone());
        if !valid {
            let mut new_line = line.split(",").collect::<Vec<&str>>();
            loop {
                let mut swapped = false;
                'main: for l in &page_rules {
                    let split = l.split("|").collect::<Vec<&str>>();
                    for i in 0..new_line.len() {
                        if new_line[i] == split[0] {
                            continue 'main;
                        }

                        if new_line[i] == split[1] {
                            for j in i..new_line.len() {
                                if new_line[j] == split[0] {
                                    new_line.swap(i, j);
                                    swapped = true;
                                    continue 'main;
                                }
                            }
                        }
                    }
                }

                if !swapped {
                    let middle = ((new_line.iter().len() / 2) as f32).ceil() as usize;
                    ret += new_line[middle].parse::<usize>()?;
                    break;
                }
            }
        }
    }

    print!("Part 2: {ret}");

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}