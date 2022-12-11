// ADVENT OF CODE 2022
// DAY 1
// COPYRIGHT (C) 2022 JAMIE MURPHY

use utils::AdventOfCode;

fn main() {
    let input = AdventOfCode::new(1);
    let data: String = input.into();
    let lines = data.lines().map(|l| l);

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
