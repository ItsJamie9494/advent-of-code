// ADVENT OF CODE 2022
// DAY 3
// COPYRIGHT (C) 2022 JAMIE MURPHY

use std::{collections::HashSet, str::Chars};

use utils::AOCInput;

fn shared_characters(a: &str, b: &str) -> Vec<char> {
    let set: HashSet<char> = a.chars().collect();
    let mut filtered_set = Vec::from_iter(b.chars().filter(|s| set.contains(&s)));
    filtered_set.dedup();

    filtered_set
}

fn alphabet_index(char: char) -> (usize, char) {
    let alphabet: Chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let (index, letter) = alphabet.enumerate().find(|s| char == s.1).unwrap();
    return (index + 1, letter);
}

fn main() {
    let input = AOCInput { date: "day3" };
    let data: String = input.into();
    let lines = data.lines();
    let lines_part_two = data.lines().collect::<Vec<&str>>();
    let mut priority: i32 = 0;
    let mut priority_part_two: i32 = 0;

    for l in lines {
        let midpoint = l.len() / 2;
        let (compartment1, compartment2) = l.split_at(midpoint);
        let shared = shared_characters(compartment1, compartment2);

        for i in shared {
            let (index, char) = alphabet_index(i);
            println!("{}, {}", index, char);
            priority += index as i32;
        }

        println!("{} & {}", compartment1, compartment2);
    }

    let chunked_lines = lines_part_two.chunks(3);
    for i in chunked_lines {
        let mut tmp = [0; 4];
        let shared_tmp = shared_characters(i.get(0).unwrap(), i.get(1).unwrap());
        let mut shared = Vec::new();
        for l in shared_tmp {
            shared.append(&mut shared_characters(
                l.encode_utf8(&mut tmp),
                i.get(2).unwrap(),
            ))
        }
        shared.dedup();

        for l in shared {
            let (index, char) = alphabet_index(l);
            println!("{}, {}", index, char);
            priority_part_two += index as i32;
            println!("{}, {}", i.get(1).unwrap(), i.get(0).unwrap());
        }
    }

    println!("\n\nThe sum of the rucksack priorities is {}", priority);

    println!("\n\nThe sum of the rucksack badges is {}", priority_part_two);
}
