use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
};

pub enum Solution {
    Part1,
    Part2,
}

#[derive(Clone)]
pub enum SolutionType {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    String(String),
}

impl Display for SolutionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::USize(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}

/// Common data for each Advent of Code day
pub struct AdventOfCode {
    pub date: i8,
    pub part1_solution: RefCell<Option<SolutionType>>,
    pub part2_solution: RefCell<Option<SolutionType>>,
}

impl Into<String> for AdventOfCode {
    // For Compatibility, should be removed soon
    fn into(self) -> String {
        self.get_input()
    }
}

impl Into<Vec<u8>> for AdventOfCode {
    fn into(self) -> Vec<u8> {
        let mut input: Vec<u8> = Vec::new();
        let mut file = File::open(format!("./day{}/input.txt", self.date)).expect("Missing Input");
        file.read_to_end(&mut input).expect("Failed to read Input");
        input
    }
}

#[allow(dead_code)]
impl AdventOfCode {
    // Create a new AdventOfCode challenge data holder
    pub fn new(day: i8) -> Self {
        Self {
            date: day,
            part1_solution: RefCell::new(None),
            part2_solution: RefCell::new(None),
        }
    }

    pub fn get_input(&self) -> String {
        let mut input: String = String::new();
        let mut file = File::open(format!("./day{}/input.txt", self.date)).expect("Missing Input");
        file.read_to_string(&mut input)
            .expect("Failed to read Input");
        input
    }

    /// Add Solution to the current AdventOfCode challenge
    pub fn add_solution(&self, solution_number: Solution, answer: SolutionType) {
        match solution_number {
            Solution::Part1 => self.part1_solution.replace(Some(answer)),
            Solution::Part2 => self.part2_solution.replace(Some(answer)),
        };
    }

    // I'll use Take here because this should be the only time the solutions are used
    pub fn print_solution_with_description(&self, descriptions: (String, String)) {
        if let Some(part1) = self.part1_solution.take() {
            println!("  {}: {}", descriptions.0, part1);
        }
        if let Some(part2) = self.part2_solution.take() {
            println!("  {}: {}", descriptions.1, part2);
        }
    }

    pub fn print_solution(&self) {
        if let Some(part1) = &self.part1_solution.take() {
            println!("  Part 1: {}", part1);
        }
        if let Some(part2) = &self.part2_solution.take() {
            println!("  Part 2: {}", part2);
        }
    }
}
