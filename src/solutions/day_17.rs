use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn solve(input: &[String]) -> String {
    let mut vertices = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, number) in row.chars().enumerate() {
            let heat_loss = number.to_digit(10).unwrap() as i32;
            for direction in Direction::iter() {
                vertices.insert(
                    (x as i32, y as i32),
                    Vertex {
                        x: x as i32,
                        y: y as i32,
                        heat_loss,
                        direction,
                    },
                );
            }
        }
    }

    let mut neighbors = HashMap::new();
    for ((x, y), vertex) in vertices.clone().into_iter() {
        let mut neighbor_values = Vec::new();
        let mut straight_cost = 0;

        for _ in 0..3 {
            let (x, y) = vertex.direction.movement(x, y);

            if !in_range(x, y, input) {
                continue;
            }

            straight_cost += vertices[&(x, y)].heat_loss;

            for turn_direction in [vertex.direction.to_left(), vertex.direction.to_right()] {
                let (x, y) = turn_direction.movement(x, y);

                if !in_range(x, y, input) {
                    continue;
                }

                let value = &vertices[&(x, y)];
                let cost = straight_cost + value.heat_loss;
                neighbor_values.push(Neighbor { value, cost })
            }
        }

        neighbors.insert(vertex, neighbor_values);
    }

    let graph = Graph { neighbors };

    format!("{}\n{}\n", part_1(&graph), part_2())
}

fn part_1(graph: &Graph) -> String {
    "part 1 unimplemented".to_string()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn in_range(x: i32, y: i32, input: &[String]) -> bool {
    0 <= x && x < input[0].len() as i32 && 0 <= y && y < input.len() as i32
}

struct Graph<'a> {
    neighbors: HashMap<Vertex, Vec<Neighbor<'a>>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vertex {
    x: i32,
    y: i32,
    heat_loss: i32,
    direction: Direction,
}

struct Neighbor<'a> {
    value: &'a Vertex,
    cost: i32,
}

#[derive(PartialEq, Eq, Hash, EnumIter, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn to_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
        }
    }

    fn to_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
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
