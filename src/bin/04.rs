use advent_of_code::*;
use anyhow::*;

const DAY: &str = "04";

const TEST_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
const TEST_INPUT_PART1_TOTAL: usize = 18;
const TEST_INPUT_PART2_TOTAL: usize = 9;

// ==== PART 1 ====
fn search_xmas(lines: Vec<Vec<char>>, chars: &Vec<char>, line_pos: usize, char_pos: usize) -> usize {
    let mut ret = 0;
    // FORWARD
    if chars.len() > char_pos + 3 {
        if chars[char_pos + 1] == 'M' && chars[char_pos + 2] == 'A' && chars[char_pos + 3] == 'S' {
            println!("Found forward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // BACKWARD
    if char_pos >= 3 {
        if chars[char_pos - 1] == 'M' && chars[char_pos - 2] == 'A' && chars[char_pos - 3] == 'S' {
            println!("Found backward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // UPWARDS
    if line_pos >= 3 {
        if lines[line_pos - 1][char_pos] == 'M' && lines[line_pos - 2][char_pos] == 'A' && lines[line_pos - 3][char_pos] == 'S' {
            println!("Found upwards XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // DOWNWARDS
    if lines.len() > line_pos + 3 {
        if lines[line_pos + 1][char_pos] == 'M' && lines[line_pos + 2][char_pos] == 'A' && lines[line_pos + 3][char_pos] == 'S' {
            println!("Found downwards XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // DIAGONAL UPWARDS FORWARD
    if line_pos >= 3 && chars.len() > char_pos + 3 {
        if lines[line_pos - 1][char_pos + 1] == 'M' && lines[line_pos - 2][char_pos + 2] == 'A' && lines[line_pos - 3][char_pos + 3] == 'S' {
            println!("Found upwards forward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // DIAGONAL UPWARDS BACKWARD
    if line_pos >= 3 && char_pos >= 3 {
        if lines[line_pos - 1][char_pos - 1] == 'M' && lines[line_pos - 2][char_pos - 2] == 'A' && lines[line_pos - 3][char_pos - 3] == 'S' {
            println!("Found upwards backward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // DIAGONAL DOWNWARDS FORWARD
    if lines.len() > line_pos + 3 && chars.len() > char_pos + 3 {
        if lines[line_pos + 1][char_pos + 1] == 'M' && lines[line_pos + 2][char_pos + 2] == 'A' && lines[line_pos + 3][char_pos + 3] == 'S' {
            println!("Found downwards forward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    // DIAGONAL DOWNWARDS BACKWARD
    if lines.len() > line_pos + 3 && char_pos >= 3 {
        if lines[line_pos + 1][char_pos - 1] == 'M' && lines[line_pos + 2][char_pos - 2] == 'A' && lines[line_pos + 3][char_pos - 3] == 'S' {
            println!("Found downwards backward XMAS at {line_pos},{char_pos}");
            ret += 1;
        }
    }

    ret
}
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let test_lines = TEST_INPUT.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let part1_solver = |lines_vec: Vec<Vec<char>>| {
        let mut total_xmases = 0;
        for (line_pos, chars) in lines_vec.iter().enumerate() {
            for (char_pos, char) in chars.iter().enumerate() {
                if *char == 'X' {
                    // we found an X! let's find every matching XMAS
                    let cloned_lines = lines_vec.clone();
                    total_xmases += search_xmas(cloned_lines, chars, line_pos, char_pos);
                }
            }
        }
        return total_xmases;
    };

    assert_eq!(part1_solver(test_lines), TEST_INPUT_PART1_TOTAL);

    // We've validated that the code tests all cases properly, run it against the real code
    let total_xmases = part1_solver(lines);

    println!("Part 1: {}", total_xmases);

    Ok(())
}

// ==== PART 2 ====
fn search_mas(lines: Vec<Vec<char>>, line_pos: usize, char_pos: usize) -> bool {
    let l_not_m: bool;
    let r_not_m: bool;

    // check what the values must be
    if lines[line_pos - 1][char_pos + 1] == 'M' {
        l_not_m = true;
    } else if lines[line_pos - 1][char_pos + 1] == 'S' {
        l_not_m = false;
    } else {
        return false;
    }

    if lines[line_pos - 1][char_pos - 1] == 'M' {
        r_not_m = true;
    } else if lines[line_pos - 1][char_pos - 1] == 'S' {
        r_not_m = false;
    } else {
        return false;
    }

    if (r_not_m && lines[line_pos + 1][char_pos + 1] != 'S') || (!r_not_m && lines[line_pos + 1][char_pos + 1] != 'M') {
        return false;
    }

    if (l_not_m && lines[line_pos + 1][char_pos - 1] != 'S') || (!l_not_m && lines[line_pos + 1][char_pos - 1] != 'M') {
        return false;
    }

    true
}

fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let test_lines = TEST_INPUT.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let part2_solver = |lines_vec: Vec<Vec<char>>| {
        let mut total_xmases = 0;
        for (line_pos, chars) in lines_vec.iter().enumerate() {
            for (char_pos, char) in chars.iter().enumerate() {
                if *char == 'A' {
                    // we found an A! let's find every matching X-MAS
                    let cloned_lines = lines_vec.clone();
                    if line_pos >= 1 && lines_vec.len() > line_pos + 1 && char_pos >= 1 && chars.len() > char_pos + 1 {
                        if search_mas(cloned_lines, line_pos, char_pos) {
                            total_xmases += 1;
                        }
                    }
                }
            }
        }
        return total_xmases;
    };

    assert_eq!(part2_solver(test_lines), TEST_INPUT_PART2_TOTAL);

    // We've validated that the code tests all cases properly, run it against the real code
    let total_xmases = part2_solver(lines);

    println!("Part 2: {}", total_xmases);

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}