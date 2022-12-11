// ADVENT OF CODE 2022
// DAY 2
// COPYRIGHT (C) 2022 JAMIE MURPHY

use utils::AdventOfCode;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
    None,
}

impl RPS {
    fn get(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
            RPS::None => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ScoreBonus {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl ScoreBonus {
    fn get(&self) -> i32 {
        match self {
            ScoreBonus::Win => 6,
            ScoreBonus::Lose => 0,
            ScoreBonus::Draw => 3,
        }
    }
}

struct Round {
    elf1: RPS,
    elf2: RPS,
    result: ScoreBonus,
    result_score: i32,
}

// Calculate the result of a round
fn score_round(results: &Vec<RPS>) -> ScoreBonus {
    if results[0] == results[1] {
        return ScoreBonus::Draw;
    } else if results[1] == RPS::Rock && results[0] != RPS::Scissors
        || results[1] == RPS::Paper && results[0] != RPS::Rock
        || results[1] == RPS::Scissors && results[0] != RPS::Paper
    {
        return ScoreBonus::Lose;
    } else {
        return ScoreBonus::Win;
    }
}

fn calculate_rps_part_one(round: &Vec<&str>) -> Round {
    let mut round_results: Vec<RPS> = Vec::new();
    let result: ScoreBonus;
    for option in round {
        let result = match *option {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => unimplemented!(),
        };
        round_results.push(result);
    }

    // Calculate result
    result = score_round(&round_results);

    // Calculate score
    Round {
        elf1: *round_results.get(0).unwrap(),
        elf2: *round_results.get(1).unwrap(),
        result: result,
        result_score: round_results[1].get() + result.get(),
    }
}

fn calculate_rps_part_two(round: &Vec<&str>) -> Round {
    let mut round_results: Vec<RPS> = Vec::new();
    let result: ScoreBonus;
    let raw_result = match round[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => unimplemented!(),
    };
    round_results.push(raw_result);
    let raw_result: RPS = match round[1] {
        "X" => match round_results[0] {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
            RPS::None => RPS::None,
        },
        "Y" => match round_results[0] {
            RPS::Rock => RPS::Rock,
            RPS::Paper => RPS::Paper,
            RPS::Scissors => RPS::Scissors,
            RPS::None => RPS::None,
        },
        "Z" => match round_results[0] {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
            RPS::None => RPS::None,
        },
        _ => RPS::None,
    };
    round_results.push(raw_result);

    // Calculate result
    result = score_round(&round_results);

    // Calculate score
    Round {
        elf1: *round_results.get(0).unwrap(),
        elf2: *round_results.get(1).unwrap(),
        result: result,
        result_score: round_results[1].get() + result.get(),
    }
}

fn main() {
    let input = AdventOfCode::new(2);
    let data: String = input.into();
    let lines = data.lines();
    let mut total_score_part_one = 0;
    let mut total_score_part_two = 0;
    for l in lines {
        let round = l.split(" ").collect::<Vec<&str>>();
        let results = calculate_rps_part_one(&round);
        println!(
            "PART ONE: {:?}, {:?}, {:?}, {}",
            results.elf1, results.elf2, results.result, results.result_score
        );

        total_score_part_one += results.result_score;

        // Part 2
        let results = calculate_rps_part_two(&round);
        println!(
            "PART TWO: {:?}, {:?}, {:?}, {}",
            results.elf1, results.elf2, results.result, results.result_score
        );

        total_score_part_two += results.result_score;
    }

    println!(
        "\n\nThe total Rock, Paper, Scissors score is {}",
        total_score_part_one
    );

    println!(
        "\n\nThe total Rock, Paper, Scissors score, following round instructions is {}",
        total_score_part_two
    );
}
