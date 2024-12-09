use advent_of_code::*;
use anyhow::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

const DAY: &str = "08";

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
const TEST_RESULT: i8 = 14;
const TEST_RESULT_2: i8 = 34;

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;
    // let test_lines = TEST_INPUT.lines().map(|x| x.to_owned()).collect::<Vec<String>>();

    let map = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // First, find the different frequencies
    let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, _) in map.iter().enumerate() {
        for (j, char) in map[i].iter().enumerate() {
            if *char != '.' {
                // Found a frequency
                if frequencies.contains_key(&char) {
                    frequencies.get_mut(&char).unwrap().push((i, j));
                } else {
                    frequencies.insert(*char, vec![(i, j)]);
                }
            }
        }
    }

    let mut result = 0;

    let mut checked: Vec<(usize, usize)> = vec![];

    // Distance
    for frequency in frequencies {
        let values = frequency.1;
        values.iter().combinations(2).map(|x| (x[0], x[1])).for_each(|(i, j)| {
            let horizontal = (i.1 as i32 - j.1 as i32).abs();
            let horizontal_sign = if i.1.cmp(&j.1) == Ordering::Greater { 1 } else { -1 };
            let vertical = (i.0 as i32 - j.0 as i32).abs();

            // Check first position
            {
                let pos_x = i.1 as i32 + (horizontal * horizontal_sign);
                let pos_y = i.0 as i32 - vertical;
                if (pos_x >= 0 && pos_x < map[0].len() as i32) &&
                    (pos_y >= 0 && pos_y < map.len() as i32) &&
                    !checked.contains(&(pos_y as usize, pos_x as usize)) {
                    checked.push((pos_y as usize, pos_x as usize));
                    result += 1;
                }
            }

            {
                let horiz_sign = if horizontal_sign == -1 { 1 } else { -1 };

                let pos_x = j.1 as i32 + (horizontal * horiz_sign);
                let pos_y = j.0 as i32 + (vertical);

                if (pos_x >= 0) && (pos_y < map.len() as i32) &&
                    (pos_y >= 0) && (pos_x < map[0].len() as i32) &&
                    !checked.contains(&(pos_y as usize, pos_x as usize)) {
                    checked.push((pos_y as usize, pos_x as usize));
                    result += 1;
                }
            }
        });
    }

    // assert_eq!(result, TEST_RESULT);
    println!("Part 1: {}", result);

    Ok(())
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?;
    // let test_lines = TEST_INPUT.lines().map(|x| x.to_owned()).collect::<Vec<String>>();

    let map = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // First, find the different frequencies
    let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, _) in map.iter().enumerate() {
        for (j, char) in map[i].iter().enumerate() {
            if *char != '.' {
                // Found a frequency
                if frequencies.contains_key(&char) {
                    frequencies.get_mut(&char).unwrap().push((i, j));
                } else {
                    frequencies.insert(*char, vec![(i, j)]);
                }
            }
        }
    }

    let mut result = 0;

    let mut checked: Vec<(usize, usize)> = vec![];

    // Distance
    for frequency in frequencies {
        let values = frequency.1;
        values.iter().combinations(2).map(|x| (x[0], x[1])).for_each(|(i, j)| {
            let horizontal = (i.1 as i32 - j.1 as i32).abs();
            let horizontal_sign = if i.1.cmp(&j.1) == Ordering::Greater { 1 } else { -1 };
            let vertical = (i.0 as i32 - j.0 as i32).abs();

            // Check first position
            {
                let mut pos_x = i.1 as i32;
                let mut pos_y = i.0 as i32;
                if !checked.contains(&(pos_y as usize, pos_x as usize)) {
                    checked.push((pos_y as usize, pos_x as usize));
                    result += 1;
                }
                loop {
                    pos_x += horizontal * horizontal_sign;
                    pos_y -= vertical;

                    if (pos_x >= 0) && (pos_y < map.len() as i32) &&
                        (pos_y >= 0) && (pos_x < map[0].len() as i32) {
                        if !checked.contains(&(pos_y as usize, pos_x as usize)) {
                            checked.push((pos_y as usize, pos_x as usize));
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
            }

            {
                let horiz_sign = if horizontal_sign == -1 { 1 } else { -1 };

                let mut pos_x = j.1 as i32;
                let mut pos_y = j.0 as i32;
                if !checked.contains(&(pos_y as usize, pos_x as usize)) {
                    checked.push((pos_y as usize, pos_x as usize));
                    result += 1;
                }
                loop {
                    pos_x += horizontal * horiz_sign;
                    pos_y += vertical;
                    if (pos_x >= 0) && (pos_y < map.len() as i32) &&
                        (pos_y >= 0) && (pos_x < map[0].len() as i32) {
                        if !checked.contains(&(pos_y as usize, pos_x as usize)) {
                            checked.push((pos_y as usize, pos_x as usize));
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        });
    }

    println!("Part 2: {}", result);

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}