use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let row_re = Regex::new(r"^([UDLR]) (\d+) \((#[0-9a-f]{6})\)$").unwrap();

    let instructions = input
        .iter()
        .map(|line| {
            let row_data = row_re.captures(line).unwrap().extract::<3>().1;
            let direction = Direction::new(row_data[0].chars().next().unwrap());
            let count = row_data[1].parse::<i64>().unwrap();
            let color = row_data[2].to_string();

            Instruction {
                direction,
                count,
                color,
            }
        })
        .collect::<Vec<_>>();

    format!("{}\n{}\n", part_1(&instructions), part_2(&instructions))
}

fn part_1(instructions: &[Instruction]) -> i64 {
    dig_and_count(
        instructions,
        |instruction| instruction.count,
        |instruction| instruction.direction.clone(),
    )
}

fn part_2(instructions: &[Instruction]) -> i64 {
    let compute_count =
        |instruction: &Instruction| i64::from_str_radix(&instruction.color[1..6], 16).unwrap();
    let compute_direction =
        |instruction: &Instruction| match instruction.color.chars().nth(6).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => unreachable!(),
        };

    dig_and_count(instructions, compute_count, compute_direction)
}

fn dig_and_count<F, G>(instructions: &[Instruction], compute_count: F, compute_direction: G) -> i64
where
    F: Fn(&Instruction) -> i64,
    G: Fn(&Instruction) -> Direction,
{
    let mut area = Area::new();

    area.dig_edges(instructions, compute_count, compute_direction);

    area.count_holes()
}

struct Area {
    corners: HashMap<i64, BTreeSet<DugCube>>,
}

impl Area {
    fn new() -> Area {
        Area {
            corners: HashMap::new(),
        }
    }

    fn dig_edges<F, G>(
        &mut self,
        instructions: &[Instruction],
        compute_count: F,
        compute_direction: G,
    ) where
        F: Fn(&Instruction) -> i64,
        G: Fn(&Instruction) -> Direction,
    {
        let (mut x, mut y) = (0, 0);

        let mut instruction_iterator = instructions.iter().peekable();

        while let Some(instruction) = instruction_iterator.next() {
            let count = compute_count(instruction);
            let current_direction = compute_direction(instruction);
            let next_direction =
                compute_direction(instruction_iterator.peek().unwrap_or(&&instructions[0]));

            match instruction.direction {
                Direction::Up | Direction::Down => {
                    self.dig_vertically(&mut x, &mut y, count, &current_direction, &next_direction)
                }
                Direction::Left | Direction::Right => self.dig_horizontally(
                    &mut x,
                    &mut y,
                    count,
                    &current_direction,
                    &next_direction,
                ),
            }
        }
    }

    fn dig_vertically(
        &mut self,
        x: &mut i64,
        y: &mut i64,
        count: i64,
        current_direction: &Direction,
        next_direction: &Direction,
    ) {
        for turn in 0..count {
            *y += match current_direction {
                Direction::Up => 1,
                Direction::Down => -1,
                _ => unreachable!(),
            };

            let r#type = if turn == count - 1 {
                match next_direction {
                    Direction::Left => DugCubeType::CornerEnd,
                    Direction::Right => DugCubeType::CornerStart,
                    _ => unreachable!(),
                }
            } else {
                DugCubeType::Edge
            };
            let vertical_direction = current_direction.clone();
            self.corners.entry(*y).or_default().insert(DugCube {
                x: *x,
                r#type,
                vertical_direction,
            });
        }
    }

    fn dig_horizontally(
        &mut self,
        x: &mut i64,
        y: &mut i64,
        count: i64,
        current_direction: &Direction,
        next_direction: &Direction,
    ) {
        *x += match current_direction {
            Direction::Left => -count,
            Direction::Right => count,
            _ => unreachable!(),
        };

        let r#type = match current_direction {
            Direction::Left => DugCubeType::CornerStart,
            Direction::Right => DugCubeType::CornerEnd,
            _ => unreachable!(),
        };
        let vertical_direction = next_direction.clone();
        self.corners.entry(*y).or_default().insert(DugCube {
            x: *x,
            r#type,
            vertical_direction,
        });
    }

    fn count_holes(&self) -> i64 {
        let mut count = self
            .corners
            .values()
            .flat_map(|edge_cubes| edge_cubes.iter())
            .count() as i64;

        for edge_cubes in self.corners.values() {
            let mut inside = false;

            for (current_cube, next_cube) in edge_cubes.iter().tuple_windows() {
                if DugCube::is_horizontal_edge(current_cube, next_cube) {
                    if current_cube.vertical_direction != next_cube.vertical_direction {
                        inside = !inside;
                    }

                    count += next_cube.x - current_cube.x - 1;
                } else {
                    inside = !inside;

                    if inside {
                        count += next_cube.x - current_cube.x - 1;
                    }
                }
            }
        }

        count
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct DugCube {
    x: i64,
    r#type: DugCubeType,
    vertical_direction: Direction,
}

impl DugCube {
    fn is_horizontal_edge(left_cube: &DugCube, right_cube: &DugCube) -> bool {
        left_cube.r#type == DugCubeType::CornerStart && right_cube.r#type == DugCubeType::CornerEnd
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
enum DugCubeType {
    Edge,
    CornerStart,
    CornerEnd,
}

struct Instruction {
    direction: Direction,
    count: i64,
    color: String,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(ch: char) -> Direction {
        match ch {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}
