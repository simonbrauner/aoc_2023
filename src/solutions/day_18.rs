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
            let count = row_data[1].parse::<i32>().unwrap();
            let _color = row_data[2].to_string();

            Instruction {
                direction,
                count,
                _color,
            }
        })
        .collect::<Vec<_>>();

    format!("{}\n{}\n", part_1(&instructions), part_2())
}

fn part_1(instructions: &[Instruction]) -> i32 {
    let mut area = Area::new();

    area.dig_edges(instructions);

    area.count_holes()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Area {
    corners: HashMap<i32, BTreeSet<DugCube>>,
}

impl Area {
    fn new() -> Area {
        Area {
            corners: HashMap::new(),
        }
    }

    fn dig_edges(&mut self, instructions: &[Instruction]) {
        let (mut x, mut y) = (0, 0);

        let mut instruction_iterator = instructions.iter().peekable();

        while let Some(instruction) = instruction_iterator.next() {
            let count = instruction.count;
            let next_direction = instruction_iterator
                .peek()
                .unwrap_or(&&instructions[0])
                .direction
                .clone();

            match instruction.direction {
                Direction::Up | Direction::Down => self.dig_vertically(
                    &mut x,
                    &mut y,
                    count,
                    &instruction.direction,
                    &next_direction,
                ),
                Direction::Left | Direction::Right => self.dig_horizontally(
                    &mut x,
                    &mut y,
                    count,
                    &instruction.direction,
                    &next_direction,
                ),
            }
        }
    }

    fn dig_vertically(
        &mut self,
        x: &mut i32,
        y: &mut i32,
        count: i32,
        direction: &Direction,
        next_direction: &Direction,
    ) {
        for turn in 0..count {
            *y += match direction {
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
            let vertical_direction = direction.clone();
            self.corners.entry(*y).or_default().insert(DugCube {
                x: *x,
                r#type,
                vertical_direction,
            });
        }
    }

    fn dig_horizontally(
        &mut self,
        x: &mut i32,
        y: &mut i32,
        count: i32,
        direction: &Direction,
        next_direction: &Direction,
    ) {
        *x += match direction {
            Direction::Left => -count,
            Direction::Right => count,
            _ => unreachable!(),
        };

        let r#type = match direction {
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

    fn count_holes(&self) -> i32 {
        let mut count = self
            .corners
            .values()
            .flat_map(|edge_cubes| edge_cubes.iter())
            .count() as i32;

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
    x: i32,
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
    count: i32,
    _color: String,
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
