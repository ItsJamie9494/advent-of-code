// ADVENT OF CODE 2022
// DAY 4
// COPYRIGHT (C) 2022 JAMIE MURPHY

use utils::AdventOfCode;

fn get_range(data: &str) -> (i32, i32) {
    data.split_once('-')
        .map(|n| (n.0.parse::<i32>().unwrap(), n.1.parse::<i32>().unwrap()))
        .expect("Data is not a range")
}

fn main() {
    let input = AdventOfCode::new(4);
    let data: String = input.into();
    let lines = data.lines();
    let mut total_contained_ranges = 0;
    let mut total_overlapping_ranges = 0;

    for line in lines {
        let (range1, range2) = line.split_once(',').expect("Data is not in pairs");
        let (start1, end1) = get_range(range1);
        let (start2, end2) = get_range(range2);

        if (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2) {
            total_contained_ranges += 1;
        }

        if (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1) {
            total_overlapping_ranges += 1;
        }
    }

    println!(
        "\n\nThe amount of contained assignment pairs is {}",
        total_contained_ranges
    );

    println!(
        "\n\nThe amount of overlapping assignment pairs is {}",
        total_overlapping_ranges
    );
}
