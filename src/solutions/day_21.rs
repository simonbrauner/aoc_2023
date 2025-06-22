use std::collections::{HashMap, HashSet};

pub fn solve(input: &[String]) -> String {
    let mut map: HashMap<Coordinate, Tile> = HashMap::new();
    let mut start_maybe = None;

    for (y, line) in input.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let coordinate = Coordinate {
                x: x as i32,
                y: y as i32,
            };

            if character == 'S' {
                start_maybe = Some(coordinate.clone());
            }

            let tile = if character == '#' {
                Tile::Rock
            } else {
                Tile::Plot
            };

            map.insert(coordinate, tile);
        }
    }

    let start = start_maybe.unwrap();

    format!("{}\n{}\n", part_1(&map, &start), part_2())
}

fn part_1(map: &HashMap<Coordinate, Tile>, start: &Coordinate) -> usize {
    let mut reachable = HashSet::new();
    reachable.insert(start.clone());

    for _ in 0..STEP_COUNT {
        let mut next_reachable = HashSet::new();

        for position in reachable.iter() {
            for neighbor in position.neighbors() {
                if let Some(tile) = map.get(&neighbor) {
                    if *tile == Tile::Plot {
                        next_reachable.insert(neighbor);
                    }
                }
            }
        }

        reachable = next_reachable;
    }

    reachable.len()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Tile {
    Plot,
    Rock,
}

impl Coordinate {
    fn neighbors(&self) -> Vec<Coordinate> {
        vec![
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

const STEP_COUNT: usize = 64;
