// ADVENT OF CODE 2022
// DAY 5
// COPYRIGHT (C) 2022 JAMIE MURPHY

use std::cell::RefCell;

use utils::AOCInput;

#[derive(Debug)]
struct Stack {
    crates: RefCell<Vec<char>>,
    // Mostly added for debugging
    _stack_id: u32,
    index: usize,
}

// Short form for storing command data
/// (`number_of_crates`, `stack_to_move_from`, `stack_to_move_to`)
type Command = (usize, usize, usize);

fn parse_stacks(crates: String) -> Vec<Stack> {
    let mut data = Vec::new();

    let lines = crates.lines();
    let crates_with_whitespace = lines.last().expect("Expected data");

    for char in crates_with_whitespace.chars() {
        if char != ' ' {
            let stack = Stack {
                crates: RefCell::new(Vec::new()),
                _stack_id: char.to_digit(10).unwrap(),
                index: crates_with_whitespace
                    .chars()
                    .position(|c| c == char)
                    .unwrap(),
            };

            let mut stack_cr = stack.crates.borrow().clone();

            for line in crates.lines() {
                stack_cr.push(line.chars().nth(stack.index).unwrap())
            }
            stack_cr.retain_mut(|c| *c != ' ');
            stack_cr.remove(stack_cr.len() - 1);
            stack.crates.replace(stack_cr);

            data.push(stack);
        }
    }

    data
}

fn parse_command(command: &str) -> Command {
    let stripped_command = command
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "");
    let data_ints = stripped_command
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (
        *data_ints.get(0).unwrap(),
        *data_ints.get(1).unwrap(),
        *data_ints.get(2).unwrap(),
    )
}

fn move_crates(stacks: &mut Vec<Stack>, command: Command) {
    for _ in 0..command.0 {
        let mut crate_to_move_from = stacks.get(command.1 - 1).unwrap().crates.borrow().clone();
        let mut crate_to_move_to = stacks.get(command.2 - 1).unwrap().crates.borrow().clone();
        let crate_to_move = crate_to_move_from.get(0).unwrap().clone();
        crate_to_move_to.insert(0, crate_to_move);
        crate_to_move_from.remove(0);
        stacks
            .get(command.1 - 1)
            .unwrap()
            .crates
            .replace(crate_to_move_from);
        stacks
            .get(command.2 - 1)
            .unwrap()
            .crates
            .replace(crate_to_move_to);
    }
}

fn move_crates_grouped(stacks: &mut Vec<Stack>, command: Command) {
    let mut crate_to_move_from = stacks.get(command.1 - 1).unwrap().crates.borrow().clone();
    let mut crate_to_move_to = stacks.get(command.2 - 1).unwrap().crates.borrow().clone();
    let mut crates_to_move = crate_to_move_from.split_at(command.0).0.to_vec();
    crates_to_move.append(&mut crate_to_move_to);
    // there has to be a better way to do this
    crate_to_move_from.reverse();
    crate_to_move_from.truncate(crate_to_move_from.len() - command.0);
    crate_to_move_from.reverse();

    stacks
        .get(command.1 - 1)
        .unwrap()
        .crates
        .replace(crate_to_move_from);
    stacks
        .get(command.2 - 1)
        .unwrap()
        .crates
        .replace(crates_to_move);
}

fn main() {
    let input = AOCInput { date: "day5" };
    let data: String = input.into();

    // Split data into Crates and Commands
    let split_data: Vec<_> = data.split("\n\n").collect();

    let (crates, commands) = (
        *split_data.first().expect("Expected data"),
        *split_data.last().expect("Expected data"),
    );

    let mut stacks = parse_stacks(crates.to_string());

    // REQUIRED FOR PART TWO, AS ORIGINAL IS MODIFIED
    let mut stacks_pt_2 = parse_stacks(crates.to_string());

    for cmd in commands.lines() {
        let command = parse_command(cmd);
        move_crates(&mut stacks, command);

        move_crates_grouped(&mut stacks_pt_2, command);
    }

    // Now handle getting the top item in each stack
    let mut result = Vec::new();
    for stack in stacks {
        let borrowed_crate = stack.crates.borrow().clone();
        let str = borrowed_crate.get(0).unwrap().clone();
        result.push(str);
    }

    let mut result_pt_2 = Vec::new();
    for stack in stacks_pt_2 {
        let borrowed_crate = stack.crates.borrow().clone();
        let str = borrowed_crate.get(0).unwrap().clone();
        result_pt_2.push(str);
    }

    println!(
        "\n\nThe top crate in each stack is {}",
        result.iter().collect::<String>()
    );

    println!(
        "\n\nThe top crate in each stack, when moved in groups is {}",
        result_pt_2.iter().collect::<String>()
    );
}
