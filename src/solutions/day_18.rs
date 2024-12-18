use std::collections::HashSet;

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
        .collect();

    format!("{}\n{}\n", part_1(&instructions), part_2())
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut area = Area::new(instructions);

    area.dig_interior();

    area.count_holes()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Area {
    holes: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Area {
    fn new(instructions: &Vec<Instruction>) -> Area {
        let (mut x, mut y) = (0, 0);
        let mut holes = HashSet::new();
        holes.insert((x, y));

        for instruction in instructions {
            for _ in 0..instruction.count {
                (x, y) = instruction.direction.movement(x, y);
                holes.insert((x, y));
            }
        }

        let min_x = holes.iter().map(|hole| hole.0).min().unwrap();
        let max_x = holes.iter().map(|hole| hole.0).max().unwrap();
        let min_y = holes.iter().map(|hole| hole.1).min().unwrap();
        let max_y = holes.iter().map(|hole| hole.1).max().unwrap();

        Area {
            holes,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn dig_interior(&mut self) {
        for y in self.min_y..=self.max_y {
            let mut inside = false;

            let mut x = self.min_x;
            while x < self.max_x {
                if self.holes.contains(&(x, y)) {
                    if self.holes.contains(&(x + 1, y)) {
                        let mut last_x = x;

                        while self.holes.contains(&(last_x + 1, y)) {
                            last_x += 1;
                        }

                        if self.holes.contains(&(x, y + 1)) != self.holes.contains(&(last_x, y + 1))
                        {
                            inside = !inside;
                        }

                        x = last_x;
                    } else {
                        inside = !inside;
                    }
                } else if inside {
                    self.holes.insert((x, y));
                }

                x += 1;
            }
        }
    }

    fn count_holes(&self) -> usize {
        self.holes.len()
    }

    fn _print_holes(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                print!(
                    "{}",
                    if self.holes.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }

            println!();
        }
    }
}

struct Instruction {
    direction: Direction,
    count: i32,
    _color: String,
}

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

    fn movement(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }
}
