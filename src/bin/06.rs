use crate::MapState::{ESCAPED, LOOP, PROGRESS};
use advent_of_code::*;
use anyhow::*;

const DAY: &str = "06";

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
enum CurrentDirection {
    UPWARDS,
    DOWNWARDS,
    LEFT,
    RIGHT,
}

#[derive(PartialEq)]
enum MapState {
    LOOP,
    PROGRESS,
    ESCAPED,
}

struct Guard {
    current_direction: CurrentDirection,
    current_position: (usize, usize),
    start_position: (usize, usize),
    _map: Vec<Vec<char>>,
    _all_positions: Vec<(usize, usize)>,
    _checked_positions: Vec<(usize, usize, CurrentDirection)>,
}

impl Guard {
    fn new(current_position: (usize, usize), map: Vec<Vec<char>>) -> Self {
        let mut obj = Self {
            current_direction: CurrentDirection::UPWARDS,
            current_position,
            start_position: current_position.clone(),
            _map: map.clone(),
            _all_positions: Vec::new(),
            _checked_positions: Vec::new(),
        };

        obj._all_positions.push(current_position.clone());
        obj._checked_positions.push((current_position.0, current_position.1, CurrentDirection::UPWARDS));

        obj
    }

    fn check_direction(&mut self) -> bool {
        match self.current_direction {
            CurrentDirection::UPWARDS => {
                if self._map[self.current_position.0 - 1][self.current_position.1] == '#' {
                    self.current_direction = CurrentDirection::RIGHT;
                    return true;
                }
            }
            CurrentDirection::DOWNWARDS => {
                if self._map[self.current_position.0 + 1][self.current_position.1] == '#' {
                    self.current_direction = CurrentDirection::LEFT;
                    return true;
                }
            }
            CurrentDirection::LEFT => {
                if self._map[self.current_position.0][self.current_position.1 - 1] == '#' {
                    self.current_direction = CurrentDirection::UPWARDS;
                    return true;
                }
            }
            CurrentDirection::RIGHT => {
                if self._map[self.current_position.0][self.current_position.1 + 1] == '#' {
                    self.current_direction = CurrentDirection::DOWNWARDS;
                    return true;
                }
            }
        }

        false
    }

    /// Pathfind. Returns whether the guard escaped the area
    fn walk(&mut self) -> bool {
        if self.check_map_maximums() {
            return true;
        }
        while self.check_direction() {}
        match self.current_direction {
            CurrentDirection::UPWARDS => {
                self.current_position = (self.current_position.0 - 1, self.current_position.1);
            }
            CurrentDirection::DOWNWARDS => {
                self.current_position = (self.current_position.0 + 1, self.current_position.1);
            }
            CurrentDirection::LEFT => {
                self.current_position = (self.current_position.0, self.current_position.1 - 1);
            }
            CurrentDirection::RIGHT => {
                self.current_position = (self.current_position.0, self.current_position.1 + 1);
            }
        }

        self._all_positions.push(self.current_position.clone());

        false
    }

    fn walk_with_check(&mut self) -> MapState {
        if self.walk() {
            return ESCAPED;
        }
        let check = (self.current_position.0, self.current_position.1, self.current_direction.clone());
        if self._checked_positions.contains(&check) {
            return LOOP;
        }
        self._checked_positions.push(check);
        PROGRESS
    }

    fn check_map_maximums(&mut self) -> bool {
        let map_maximums = (self._map[0].len(), self._map[1].len());

        if self.current_position.0 + 1 == map_maximums.0 || self.current_position.1 + 1 == map_maximums.1 ||
            (self.current_position.0 as i32) - 1 < 0 || (self.current_position.1 as i32) - 1 < 0 {
            return true;
        }

        false
    }

    fn set_obstacle(&mut self, obstacle_pos: (usize, usize)) -> bool {
        if self._map[obstacle_pos.0][obstacle_pos.1] == '#' {
            false
        } else if self._map[obstacle_pos.0][obstacle_pos.1] == '^' {
            false
        } else {
            self._map[obstacle_pos.0][obstacle_pos.1] = '#';
            true
        }
    }

    fn remove_obstacle(&mut self, obstacle_pos: (usize, usize)) {
        self._map[obstacle_pos.0][obstacle_pos.1] = '.';
    }

    fn clear(&mut self) {
        self.current_direction = CurrentDirection::UPWARDS;
        self.current_position = self.start_position.clone();
        self._checked_positions.clear();
        self._all_positions.clear();
    }
}

// ==== PART 1 ====
fn part1() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    // (line_num, char_pos)
    let mut starting_position: (usize, usize) = (0, 0);

    for (line_num, l) in lines.iter().enumerate() {
        if let Some(position) = l.chars().position(|c| c == '^') {
            starting_position = (line_num, position);
            break;
        }
    }

    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();

    let mut guard = Guard::new(starting_position, map);

    while !guard.walk() {}

    let mut cleaned_positions = guard._all_positions.clone();
    cleaned_positions.sort();
    cleaned_positions.dedup();
    println!("Part 1: {:?}", cleaned_positions.len());

    Ok(())
}

// ==== PART 2 ====
fn part2() -> Result<()> {
    let lines = input_as_lines(DAY)?;

    let mut starting_position: (usize, usize) = (0, 0);

    for (line_num, l) in lines.iter().enumerate() {
        if let Some(position) = l.chars().position(|c| c == '^') {
            starting_position = (line_num, position);
            break;
        }
    }

    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();

    let mut guard = Guard::new(starting_position, map);

    let mut loops = 0;
    for i in 0..guard._map.len() {
        'inner: for j in 0..guard._map[i].len() {
            println!("Testing ({i}, {j})");
            if guard.set_obstacle((i, j)) {
                loop {
                    let state = guard.walk_with_check();
                    if state == ESCAPED {
                        guard.remove_obstacle((i, j));
                        guard.clear();
                        continue 'inner;
                    } else if state == PROGRESS {
                        continue;
                    } else if state == LOOP {
                        guard.remove_obstacle((i, j));
                        guard.clear();
                        loops += 1;
                        continue 'inner;
                    }
                }
            }
        }
    }

    println!("Part 2: {:?}", loops);

    Ok(())
}

fn main() -> Result<()> {
    start_day(DAY, &part1, &part2)
}