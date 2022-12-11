// ADVENT OF CODE 2022
// DAY 8
// ADAPTED FROM https://github.com/gbegerow/advent-of-code/blob/main/aoc_2022_08/src/lib.rs
// i am behind and don't really want to spend time figuring out algorithms

use std::collections::HashSet;

use utils::{AdventOfCode, Solution, SolutionType};

#[derive(Clone)]
struct Grid {
    width: i32,
    height: i32,
    map: Vec<u32>,
}

/// A representation of a Point on the grid
struct Point {
    // Width
    w: usize,
    // Height
    h: usize,
    // X-coordinate
    x: usize,
    // Y-coordinate
    y: usize,
}

impl Grid {
    pub fn fill(input: &str) -> Self {
        let mut map = Vec::new();
        let mut width: i32 = 0;
        let mut height = 0;

        for line in input.lines() {
            let mut row: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
            // We're adding a row
            height += 1;
            if width < row.len().try_into().unwrap() {
                width = row.len().try_into().unwrap();
            }
            map.append(&mut row);
        }

        Self { width, height, map }
    }

    // Get a point on the grid
    pub fn get(&self, point: &Point) -> u32 {
        let index = (point.w * point.y) + point.x;
        self.map[index]
    }

    // Check if a point if visible
    pub fn visible(self, point: &Point) -> bool {
        let cell = self.get(point);

        // Outer edge points are always visible
        let mut visible =
            point.x == 0 || point.y == 0 || point.x == point.w - 1 || point.y == point.h - 1;

        if !visible {
            // Visible from LTR
            visible = visible
                || self.map[point.w * point.y..point.w * point.y + point.x]
                    .iter()
                    .all(|t| t < &cell);
            visible = visible
                || self.map[point.w * point.y + point.x + 1..point.w * (point.y + 1)]
                    .iter()
                    .all(|t| t < &cell);

            // Visible from Up to Down
            visible = visible || (0..point.y).all(|y| self.map[point.w * y + point.x] < cell);
            visible =
                visible || (point.y + 1..point.h).all(|y| self.map[point.w * y + point.x] < cell)
        }

        visible
    }

    pub fn scenic_score(self, point: &Point) -> u32 {
        let cell = self.get(point);

        if point.x == 0 || point.y == 0 || point.x == point.w - 1 || point.y == point.h - 1 {
            return 0;
        }

        let mut score = 1;

        let mut run = 0;
        for t in (point.w * point.y..point.w * point.y + point.x).rev() {
            run += 1;
            if self.map[t] >= cell {
                break;
            }
        }
        score *= run;

        run = 0;
        for t in point.w * point.y + point.x + 1..point.w * (point.y + 1) {
            run += 1;
            if self.map[t] >= cell {
                break;
            }
        }
        score *= run;

        run = 0;
        for t in (0..point.y).rev() {
            run += 1;
            if self.map[point.w * t + point.x] >= cell {
                break;
            }
        }
        score *= run;

        run = 0;
        for t in point.y + 1..point.h {
            run += 1;
            if self.map[point.w * t + point.x] >= cell {
                break;
            }
        }
        score *= run;

        score
    }
}

fn main() {
    let aoc = AdventOfCode::new(8);
    let grid = Grid::fill(&aoc.get_input());
    let mut visible = HashSet::new();
    let mut max_score = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point {
                w: grid.width.try_into().unwrap(),
                h: grid.height.try_into().unwrap(),
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            if grid.clone().visible(&point) {
                visible.insert((x, y));
            }

            let score = grid.clone().scenic_score(&point);
            if score > max_score {
                max_score = score;
            }
        }
    }

    aoc.add_solution(Solution::Part1, SolutionType::USize(visible.len()));
    aoc.add_solution(Solution::Part2, SolutionType::U32(max_score));

    aoc.print_solution();
}
