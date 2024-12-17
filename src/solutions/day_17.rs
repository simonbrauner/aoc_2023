use std::{cmp::Reverse, collections::HashMap};

use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn solve(input: &[String]) -> String {
    let x_size = input[0].len() as i32;
    let y_size = input.len() as i32;
    let mut vertices = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, number) in row.chars().enumerate() {
            let heat_loss = number.to_digit(10).unwrap() as i32;
            for direction in Direction::iter() {
                vertices.insert(
                    (x as i32, y as i32, direction.clone()),
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
    for ((mut x, mut y, _), vertex) in vertices.clone().into_iter() {
        let mut neighbor_values = Vec::new();
        let mut cost = 0;

        for _ in 0..3 {
            (x, y) = vertex.direction.movement(x, y);

            if !in_range(x, y, x_size, y_size) {
                break;
            }

            cost += vertices[&(x, y, Direction::Right)].heat_loss;

            for turn_direction in [vertex.direction.to_left(), vertex.direction.to_right()] {
                let (turn_x, turn_y) = turn_direction.movement(x, y);

                if !in_range(turn_x, turn_y, x_size, y_size) {
                    continue;
                }

                let value = &vertices[&(x, y, turn_direction)];
                neighbor_values.push(Neighbor { value, cost })
            }
        }

        neighbors.insert(vertex, neighbor_values);
    }

    let graph = Graph { neighbors };
    let start = &vertices[&(0, 0, Direction::Right)];
    let destination = &vertices[&(x_size - 1, y_size - 1, Direction::Right)];

    format!("{}\n{}\n", part_1(&graph, start, destination), part_2())
}

fn part_1(graph: &Graph, start: &Vertex, destination: &Vertex) -> i32 {
    let mut distances: HashMap<_, _> = graph
        .neighbors
        .keys()
        .map(|vertex| (vertex, i32::MAX))
        .collect();
    distances.insert(start, 0);

    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(0));

    while let Some((vertex, _)) = queue.pop() {
        for neighbor in graph.neighbors.get(vertex).unwrap() {
            let shorter_distance = distances.get(vertex).unwrap() + neighbor.cost;
            if shorter_distance < *distances.get(neighbor.value).unwrap() {
                distances.insert(neighbor.value, shorter_distance);
                queue.push_increase(neighbor.value, Reverse(shorter_distance));
            }
        }
    }

    Direction::iter()
        .map(|direction| {
            *distances
                .get(&Vertex {
                    direction,
                    ..destination.clone()
                })
                .unwrap()
        })
        .min()
        .unwrap()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn in_range(x: i32, y: i32, x_size: i32, y_size: i32) -> bool {
    0 <= x && x < x_size as i32 && 0 <= y && y < y_size as i32
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
