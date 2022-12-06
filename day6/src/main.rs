// ADVENT OF CODE 2022
// DAY 6
// COPYRIGHT (C) 2022 JAMIE MURPHY

use utils::AOCInput;

// Determines if characters in a string are unique
fn unique_characters(a: &str) -> bool {
    let mut vec: Vec<char> = a.chars().collect();
    vec.sort_by(|a, b| a.cmp(b));

    vec.dedup();
    vec.len() == a.len()
}

fn find_marker(marker_length: usize, characters: &String) -> (String, usize) {
    let mut pos: usize = 0;
    let mut return_value: (String, usize) = (String::new(), 0);

    for _ in characters.chars() {
        let end_pos = pos + marker_length;
        let current_set = characters.get(pos..end_pos).unwrap();
        if unique_characters(current_set) == true {
            return_value = (current_set.to_string(), end_pos);
            break;
        }
        pos += 1;
    }

    return_value
}

fn main() {
    let input = AOCInput { date: "day6" };
    let data: String = input.into();

    println!(
        "\n\nThe start-of-packet marker, {}, is found at {}",
        find_marker(4, &data).0,
        find_marker(4, &data).1
    );

    println!(
        "\n\nThe start-of-message marker, {}, is found at {}",
        find_marker(14, &data).0,
        find_marker(14, &data).1
    );
}
