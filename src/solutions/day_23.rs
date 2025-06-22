use std::{
    cmp,
    collections::{HashMap, HashSet},
};

pub fn solve(input: &[String]) -> String {
    let mut map: HashMap<Coordinate, Tile> = HashMap::new();
    let start = Coordinate { x: 1, y: 0 };
    let end = Coordinate {
        x: input[0].len() as i32 - 2,
        y: input.len() as i32 - 1,
    };

    for (y, line) in input.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let coordinate = Coordinate {
                x: x as i32,
                y: y as i32,
            };
            let tile = Tile::new(character);
            map.insert(coordinate, tile);
        }
    }

    format!("{}\n{}\n", part_1(&map, &start, &end), part_2())
}

fn part_1(map: &HashMap<Coordinate, Tile>, start: &Coordinate, end: &Coordinate) -> i32 {
    longest_path_length(map, end, start.clone(), &mut HashSet::new())
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

fn longest_path_length(
    map: &HashMap<Coordinate, Tile>,
    end: &Coordinate,
    position: Coordinate,
    visited: &mut HashSet<Coordinate>,
) -> i32 {
    if position == *end {
        return visited.len() as i32;
    }

    if !visited.insert(position.clone()) {
        return 0;
    }

    let mut steps = 0;

    for (neighbor, direction) in position.neighbors() {
        if let Some(tile) = map.get(&neighbor) {
            if tile.can_enter(&direction) {
                steps = cmp::max(steps, longest_path_length(map, end, neighbor, visited))
            }
        }
    }

    visited.remove(&position);

    steps
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Other,
}

impl Coordinate {
    fn neighbors(&self) -> Vec<(Coordinate, Direction)> {
        vec![
            (
                Coordinate {
                    x: self.x + 1,
                    y: self.y,
                },
                Direction::Right,
            ),
            (
                Coordinate {
                    x: self.x - 1,
                    y: self.y,
                },
                Direction::Other,
            ),
            (
                Coordinate {
                    x: self.x,
                    y: self.y + 1,
                },
                Direction::Down,
            ),
            (
                Coordinate {
                    x: self.x,
                    y: self.y - 1,
                },
                Direction::Other,
            ),
        ]
    }
}

enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '>' => Tile::Slope(Direction::Right),
            'v' => Tile::Slope(Direction::Down),
            _ => unreachable!(),
        }
    }

    fn can_enter(&self, dir: &Direction) -> bool {
        match self {
            Tile::Path => true,
            Tile::Forest => false,
            Tile::Slope(coord_dir) => coord_dir == dir,
        }
    }
}
