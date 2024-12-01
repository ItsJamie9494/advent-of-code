use advent_of_code::*;
use anyhow::*;

const DAY: &str = "01";

fn get_input() -> Result<(Vec<String>, Vec<String>)> {
    let lines = input_as_lines(DAY)?;

    let mut list1: Vec<String> = vec![]; // First Row
    let mut list2: Vec<String> = vec![]; // Second Row
    for line in lines {
        let mut lists = line.split_whitespace();
        list1.push(lists.next().unwrap().to_string());
        list2.push(lists.next().unwrap().to_string());
    }

    // Sort Lists
    list1.sort();
    list2.sort();

    // Sanity Check
    if list1.len() != list2.len() {
        panic!("lists do not match");
    }

    Ok((list1, list2))
}

fn part1() -> Result<()> {
    let (list1, list2) = get_input()?;

    let mut total_distance = 0;

    for (loc, elem1) in list1.iter().enumerate() {
        let elem2 = list2.iter().nth(loc).unwrap();
        let distance = (elem1.parse::<i32>()? - elem2.parse::<i32>()?).abs();

        total_distance += distance;
    }

    println!("Total Distance: {}", total_distance);

    Ok(())
}

fn part2() -> Result<()> {
    let (list1, list2) = get_input()?;

    let mut similarity_score: i32 = 0;
    for elem in &list1 {
        let num = elem.parse::<i32>()?;

        let mut matches = 0;
        for elem2 in &list2 {
            let num2 = elem2.parse::<i32>()?;
            if num2 == num {
                matches += 1;
            } else if num2 != num && matches > 0 { // The lists are both sorted
                break;
            }
        }

        similarity_score += num * matches;
    }

    println!("Similarity Score: {}", similarity_score);

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}