use advent_of_code::*;
use anyhow::*;

const DAY: &str = "02";

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mut safe_lines = lines.len();
    lines.iter().for_each(|line| {
        let mut line_increasing: bool = false;
        line.split_whitespace().zip(line.split_whitespace().skip(1)).enumerate().map(|(pos, (first, second))| {
            let i1 = first.parse::<i32>().unwrap();
            let i2 = second.parse::<i32>().unwrap();
            if (i1 - i2).abs() > 3 || (i1 - i2).abs() < 1 {
                return false;
            }

            return if pos > 0 {
                (i2 > i1 && line_increasing) || (i2 < i1 && !line_increasing)
            } else {
                line_increasing = i2.gt(&i1);
                true
            };
        }).collect::<Vec<bool>>().contains(&false).then(|| safe_lines -= 1);
    });

    println!("Safe Reports: {safe_lines}");

    Ok(())
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let line_safety_checker = |line: &str| {
        let mut line_increasing: bool = false;
        !line.split_whitespace().zip(line.split_whitespace().skip(1)).enumerate().map(|(pos, (first, second))| {
            let i1 = first.parse::<i32>().unwrap();
            let i2 = second.parse::<i32>().unwrap();
            if (i1 - i2).abs() > 3 || (i1 - i2).abs() < 1 {
                return false;
            }

            return if pos > 0 {
                (i2 > i1 && line_increasing) || (i2 < i1 && !line_increasing)
            } else {
                line_increasing = i2.gt(&i1);
                true
            };
        }).collect::<Vec<bool>>().contains(&false)
    };

    let mut safe_lines = 0;
    lines.iter().for_each(|line| {
        if !line_safety_checker(line) {
            let mut new_safe_lines = vec![];
            for i in 0..line.split_whitespace().collect::<Vec<&str>>().len() {
                let mut l = line.split_whitespace().collect::<Vec<&str>>();
                l.remove(i);
                let removed_line = l.join(" ");

                new_safe_lines.push(line_safety_checker(&removed_line));
            }

            new_safe_lines.contains(&true).then(|| safe_lines += 1);
        } else {
            safe_lines += 1;
        }
    });

    println!("Safe Reports: {safe_lines}");


    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}