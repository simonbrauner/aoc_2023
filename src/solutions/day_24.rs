use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let number_re = Regex::new(r"-?\d+").unwrap();

    let hailstones: Vec<_> = input
        .iter()
        .map(|line| {
            let numbers: Vec<_> = number_re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();

            Hailstone {
                x: numbers[0],
                y: numbers[1],
                dx: numbers[3],
                dy: numbers[4],
            }
        })
        .collect();

    format!("{}\n{}\n", part_1(&hailstones), part_2())
}

fn part_1(hailstones: &[Hailstone]) -> usize {
    hailstones
        .iter()
        .combinations(2)
        .filter(|pair| pair[0].will_cross_inside(pair[1]))
        .count()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Hailstone {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

impl Hailstone {
    fn will_cross_inside(&self, other: &Self) -> bool {
        let matrix = Matrix2::new(self.dx, -other.dx, self.dy, -other.dy);
        let vector = Vector2::new(-self.x + other.x, -self.y + other.y);

        if let Some(solution) = matrix.lu().solve(&vector) {
            let x = self.x + solution[0] * self.dx;
            let y = self.y + solution[0] * self.dy;

            (MIN_COORDINATE..=MAX_COORDINATE).contains(&x)
                && (MIN_COORDINATE..=MAX_COORDINATE).contains(&y)
                && solution[0] >= 0.0
                && solution[1] >= 0.0
        } else {
            false
        }
    }
}

const MIN_COORDINATE: f64 = 200000000000000.0;
const MAX_COORDINATE: f64 = 400000000000000.0;
