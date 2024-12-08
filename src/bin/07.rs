use advent_of_code::*;
use anyhow::*;

const DAY: &str = "07";

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mut total_calibration = 0;
    for i in lines {
        let data = i.split(": ").collect::<Vec<&str>>();
        let calibrator = data[0].parse::<u64>()?;
        let inputs = data[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        // Permutation, 2^N
        for mut i in 0..2_u64.pow(u32::try_from(inputs.len() as u64 - 1)?) {
            let mut numbers = inputs.iter();
            let mut test_result = *numbers.next().unwrap();
            for n in numbers {
                if i & 1 == 0 {
                    test_result += n;
                } else {
                    test_result *= n;
                }
                // bitwise operations are so annoying
                i >>= 1;
            }

            if test_result == calibrator {
                total_calibration += test_result;
                break;
            }
        }
    }

    println!("Part 1: {total_calibration}");

    Ok(())
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?;
    // these damn elephants

    let mut total_calibration = 0;
    for i in lines {
        let data = i.split(": ").collect::<Vec<&str>>();
        let calibrator = data[0].parse::<u64>()?;
        let inputs = data[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        // 3^N :(
        for mut i in 0..3_u64.pow(u32::try_from(inputs.len() as u64 - 1)?) {
            let mut numbers = inputs.iter();
            let mut test_result = *numbers.next().unwrap();
            for n in numbers {
                if i % 3 == 0 {
                    test_result += n;
                } else if i % 3 == 1 {
                    test_result *= n;
                    test_result *= 10_u64.pow(n.ilog10() + 1);
                    test_result += n;
                }
                i /= 3;
            }

            if test_result == calibrator {
                total_calibration += test_result;
                break;
            }
        }
    }

    println!("Part 2: {total_calibration}");


    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}