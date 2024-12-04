use advent_of_code::*;
use anyhow::*;
use regex::Regex;

const DAY: &str = "03";

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mal_re = Regex::new(
        r#"mul\(\d*,\d*\)"#
    )?;

    let mut results: Vec<String> = vec![];
    for i in lines {
        mal_re.find_iter(&i).map(|c| c).for_each(|c| results.push(c.as_str().to_string()));
    }

    // perform operations
    let mut resultant = 0;
    for i in results {
        let ints = i.strip_prefix("mul(").unwrap_or(i.as_str()).strip_suffix(")").unwrap_or(i.as_str()).split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let calculation = ints[0] * ints[1];
        resultant += calculation;
    }

    println!("Resultant Value: {}", resultant);

    Ok(())
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let input = input_as_string(DAY)?;

    let mal_re = Regex::new(
        r#"mul\(\d*,\d*\)"#
    )?;

    let mut safe_input = input.clone();
    let _ = input.match_indices("don't()").zip(input.match_indices("don't()").skip(1)).enumerate().for_each(|(_, (y, z))| {
        let line = input.get(y.0..z.0).unwrap_or("");
        if let Some(loc) = line.find("do()") {
            safe_input = safe_input.replace(input.get(y.0..y.0 + loc).unwrap(), "");
        } else {
            safe_input = safe_input.replace(line, "");
        }
    });

    // The above code doesn't account for the last `don't()`, so remove the last one (if there is one)
    safe_input.clone().match_indices("don't()").for_each(|(y, _)| {
        let line = safe_input.get(y..safe_input.len()).unwrap_or("");
        safe_input = safe_input.replace(line, "");
    });

    let mut results: Vec<String> = vec![];
    mal_re.find_iter(&safe_input).map(|c| c).for_each(|c| results.push(c.as_str().to_string()));

    // perform operations
    let mut resultant = 0;
    for i in results {
        let ints = i.strip_prefix("mul(").unwrap_or(i.as_str()).strip_suffix(")").unwrap_or(i.as_str()).split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let calculation = ints[0] * ints[1];
        resultant += calculation;
    }

    println!("Resultant Value: {}", resultant);

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}