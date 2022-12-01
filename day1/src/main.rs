// ADVENT OF CODE 2022
// DAY 1
// COPYRIGHT (C) 2022 JAMIE MURPHY

use std::{
    fs::File,
    io::{BufRead, Read},
};

fn get_input() -> Vec<u8> {
    let mut vec = Vec::new();
    let mut file = File::open("./day1/input.txt").expect("Missing Program Input");
    file.read_to_end(&mut vec).expect("Could not read file");
    vec
}

fn main() {
    let input = get_input();
    let lines = input.lines().map(|l| l.unwrap());

    // Add input together
    let mut data = Vec::new();
    let mut temp_data: i32 = 0;
    for l in lines {
        if !l.is_empty() {
            temp_data += l.parse::<i32>().unwrap();
        } else {
            data.push(temp_data);
            temp_data = 0;
        }
    }

    // Sort input by largest to smallest
    data.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Print largest
    println!(
        "\n\nThe Elf with the largest calories has {} calories",
        data.first().expect("Empty data")
    );

    // PART TWO
    ///////////

    // Since all data is sorted, this is super easy :)
    let mut top_three_total: i32 = 0;
    for i in 0..3 {
        top_three_total += data.get(i).unwrap();
    }

    println!(
        "\n\nThe Top Three Elves have {} calories combined",
        top_three_total
    );
}
