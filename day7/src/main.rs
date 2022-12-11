// ADVENT OF CODE 2022
// DAY 7
// COPYRIGHT (C) 2022 JAMIE MURPHY

use utils::{AdventOfCode, Solution, SolutionType};

// Root is always index 0
const ROOT: usize = 0;
const MAX_SPACE_AVAILABLE: i32 = 70000000;
const MIN_SPACE_TO_DELETE: i32 = 30000000;

#[allow(dead_code)]
pub struct Directory {
    name: String,
    size: i32,
    parent: usize,
}

#[derive(Debug)]
pub enum Command {
    File(i32, String),
    Cd(String),
    None,
}

impl Command {
    fn parse(cmd: &str) -> Self {
        if !cmd.contains("$") {
            let mut split_cmd = cmd.split_whitespace();
            let _type = split_cmd
                .clone()
                .next()
                .unwrap()
                .chars()
                .any(|c| c.is_numeric());

            if _type == true {
                Self::File(
                    split_cmd.nth(0).unwrap().parse::<i32>().unwrap(),
                    split_cmd.next().unwrap().to_string(),
                )
            } else {
                Self::None
            }
        } else {
            let parsed_cmd = cmd.replace("$ ", "");
            let mut split_cmd = parsed_cmd.split_whitespace();
            let cmd_str = split_cmd.nth(0).unwrap().to_string();
            if cmd_str == "cd" {
                Self::Cd(split_cmd.next().unwrap().to_string())
            } else {
                Self::None
            }
        }
    }
}

fn main() {
    let input = AdventOfCode::new(7);
    let data: String = input.get_input();
    let lines = data.lines();

    let mut dirs = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];
    // Store Index
    let mut cwd = ROOT;

    for line in lines {
        let cmd = Command::parse(line);
        match cmd {
            Command::File(size, _name) => {
                let mut path = cwd;
                loop {
                    dirs[path].size += size;
                    if path == ROOT {
                        break;
                    }
                    path = dirs[path].parent;
                }
            }
            // I hate myself for doing &* but I need a &str
            Command::Cd(dir) => match &*dir {
                "/" => cwd = ROOT,
                ".." => cwd = dirs[cwd].parent,
                name => {
                    dirs.push(Directory {
                        name: String::from(name),
                        size: 0,
                        parent: cwd,
                    });
                    cwd = dirs.len() - 1; // Cwd is now last entry
                }
            },
            // Do nothing, but don't call any panics or unimplemented errors.
            // It's intentionally unimplemented
            Command::None => {}
        }
    }

    input.add_solution(
        Solution::Part1,
        SolutionType::I32(
            dirs.iter()
                .map(|d| d.size)
                .filter(|size| *size <= 100000)
                .sum::<i32>(),
        ),
    );

    let free_space = MAX_SPACE_AVAILABLE - dirs[0].size;
    let need_to_free = MIN_SPACE_TO_DELETE - free_space;

    let _size = dirs[0].size;
    input.add_solution(
        Solution::Part2,
        SolutionType::I32(
            dirs.iter()
                .map(|d| d.size)
                .filter(|size| *size >= need_to_free)
                .min()
                .expect("At least one directory must be deleted"),
        ),
    );

    input.print_solution();
}
