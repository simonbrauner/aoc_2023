use std::cmp;

use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let number_re = Regex::new(r"\d+").unwrap();

    let bricks: Vec<_> = input
        .iter()
        .map(|line| {
            let numbers: Vec<_> = number_re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            Brick {
                x_min: cmp::min(numbers[0], numbers[3]),
                x_max: cmp::max(numbers[0], numbers[3]),
                y_min: cmp::min(numbers[1], numbers[4]),
                y_max: cmp::max(numbers[1], numbers[4]),
                z_min: cmp::min(numbers[2], numbers[5]),
                z_max: cmp::max(numbers[2], numbers[5]),
            }
        })
        .collect();

    format!("{}\n{}\n", part_1(bricks.clone()), part_2())
}

fn part_1(bricks: Vec<Brick>) -> usize {
    let bricks_on_ground = make_bricks_fall(bricks);

    bricks_on_ground
        .clone()
        .into_iter()
        .filter(|removed| {
            let bricks_without_removed: Vec<_> = bricks_on_ground
                .clone()
                .into_iter()
                .filter(|b| b != removed)
                .collect();

            bricks_without_removed
                .iter()
                .all(|brick| !brick.can_move_down(&bricks_without_removed))
        })
        .count()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn make_bricks_fall(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut bricks = bricks;
    let mut change = true;

    while change {
        let mut next_bricks = Vec::new();
        change = false;

        for brick in bricks.iter() {
            let mut next_brick = brick.clone();

            if next_brick.can_move_down(&bricks) {
                next_brick.move_down();
                change = true;
            }

            next_bricks.push(next_brick);
        }

        bricks = next_bricks;
    }

    bricks
}

#[derive(Clone, PartialEq)]
struct Brick {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    z_min: usize,
    z_max: usize,
}

impl Brick {
    fn can_move_down(&self, bricks: &Vec<Brick>) -> bool {
        if self.z_min == 1 {
            return false;
        }

        for brick in bricks {
            if self.z_min - 1 == brick.z_max
                && cmp::max(self.x_min, brick.x_min) <= cmp::min(self.x_max, brick.x_max)
                && cmp::max(self.y_min, brick.y_min) <= cmp::min(self.y_max, brick.y_max)
            {
                return false;
            }
        }

        true
    }

    fn move_down(&mut self) {
        self.z_min -= 1;
        self.z_max -= 1;
    }
}
