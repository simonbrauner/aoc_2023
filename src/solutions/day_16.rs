use std::collections::{HashSet, VecDeque};

pub fn solve(input: &[String]) -> String {
    let grid = Grid {
        rows: input
            .iter()
            .map(|line| line.chars().map(Square::new).collect())
            .collect(),
    };

    format!("{}\n{}\n", part_1(grid), part_2())
}

fn part_1(mut grid: Grid) -> usize {
    grid.move_beams();

    grid.count_energized()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Grid {
    rows: Vec<Vec<Square>>,
}

impl Grid {
    fn x_size(&self) -> usize {
        self.rows[0].len()
    }

    fn y_size(&self) -> usize {
        self.rows.len()
    }

    fn square_at_coords(&mut self, x: usize, y: usize) -> &mut Square {
        &mut self.rows[y][x]
    }

    fn move_beams(&mut self) {
        let mut queue = VecDeque::from(vec![(0, 0, Direction::Right)]);
        self.square_at_coords(0, 0).energize(Direction::Right);

        while let Some((x, y, direction)) = queue.pop_front() {
            for next_direction in self
                .square_at_coords(x, y)
                .next_directions(direction.clone())
            {
                let mut next_x = x;
                let mut next_y = y;

                if next_direction == Direction::Up && y != 0 {
                    next_y = y - 1;
                } else if next_direction == Direction::Down && y != self.y_size() - 1 {
                    next_y = y + 1;
                } else if next_direction == Direction::Left && x != 0 {
                    next_x = x - 1;
                } else if next_direction == Direction::Right && x != self.x_size() - 1 {
                    next_x = x + 1;
                } else {
                    continue;
                }

                let next_square = self.square_at_coords(next_x, next_y);
                if !next_square.visited_from.contains(&next_direction) {
                    next_square.energize(next_direction.clone());
                    queue.push_back((next_x, next_y, next_direction));
                }
            }
        }
    }

    fn count_energized(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .map(|square| if square.is_energized() { 1 } else { 0 })
            .sum()
    }
}

struct Square {
    r#type: SquareType,
    visited_from: HashSet<Direction>,
}

impl Square {
    fn new(ch: char) -> Square {
        Square {
            r#type: SquareType::new(ch),
            visited_from: HashSet::new(),
        }
    }

    fn energize(&mut self, direction: Direction) {
        self.visited_from.insert(direction);
    }

    fn is_energized(&self) -> bool {
        !self.visited_from.is_empty()
    }

    fn next_directions(&self, direction: Direction) -> Vec<Direction> {
        match self.r#type {
            SquareType::EmptySpace => vec![direction],
            SquareType::ForwardMirror => vec![match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }],
            SquareType::BackMirror => vec![match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }],
            SquareType::UpDownSplitter => match direction {
                Direction::Up | Direction::Down => vec![direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            SquareType::LeftRightSplitter => match direction {
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left | Direction::Right => vec![direction],
            },
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum SquareType {
    EmptySpace,
    ForwardMirror,
    BackMirror,
    UpDownSplitter,
    LeftRightSplitter,
}

impl SquareType {
    fn new(ch: char) -> SquareType {
        match ch {
            '.' => SquareType::EmptySpace,
            '/' => SquareType::ForwardMirror,
            '\\' => SquareType::BackMirror,
            '|' => SquareType::UpDownSplitter,
            '-' => SquareType::LeftRightSplitter,
            _ => unreachable!(),
        }
    }
}
