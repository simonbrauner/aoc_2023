use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &[String]) -> String {
    format!("{}\n{}\n", part_1(input), part_2(input))
}

fn part_1(pipes: &[String]) -> usize {
    main_loop_coordinates(pipes).len() / 2
}

fn part_2(pipes: &[String]) -> usize {
    let maze = pipes_maze(pipes, &main_loop_coordinates(pipes));

    let max_x = maze[0].len() - 2;
    let max_y = maze.len() - 2;

    let mut inside_outside = HashMap::new();

    for y in 1..=max_y {
        for x in 1..=max_x {
            let character = maze[y][x];

            if inside_outside.contains_key(&(x, y)) || character == '.' {
                continue;
            }

            explore_tiles(x, y, &maze, &mut inside_outside);
        }
    }

    let mut inside_count = 0;

    for y in 1..=max_y {
        for x in 1..=max_x {
            if x % 2 == 1 && y % 2 == 1 {
                if let Some(EmptyTileType::Inside) = inside_outside.get(&(x, y)) {
                    inside_count += 1;
                }
            }
        }
    }

    inside_count
}

#[derive(PartialEq, Clone)]
enum EmptyTileType {
    Inside,
    Outside,
    Unknown,
}

fn first_move(pipes: &[String]) -> (isize, isize) {
    let (col_start, row_start) = start_coordinates(pipes);

    for (col_forward, row_forward) in possible_directions(START) {
        let (Some(col_neighbor), Some(row_neighbor)) = (
            col_start.checked_add_signed(col_forward),
            row_start.checked_add_signed(row_forward),
        ) else {
            continue;
        };

        if let Some(row) = pipes.get(row_neighbor) {
            if let Some(pipe) = row.chars().nth(col_neighbor) {
                for (col_backward, row_backward) in possible_directions(pipe) {
                    if is_opposite_direction(
                        (col_backward, row_backward),
                        (col_forward, row_forward),
                    ) {
                        return (col_forward, row_forward);
                    }
                }
            }
        }
    }

    unreachable!()
}

fn start_coordinates(pipes: &[String]) -> (usize, usize) {
    for (row_index, row) in pipes.iter().enumerate() {
        if let Some(col_index) = row.find(START) {
            return (col_index, row_index);
        }
    }

    unreachable!()
}

const START: char = 'S';
const OUTSIDE: char = 'O';
const REAL_START: char = '7';

const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const EAST: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (-1, 0);

fn possible_directions(pipe: char) -> Vec<(isize, isize)> {
    match pipe {
        '|' => vec![NORTH, SOUTH],
        '-' => vec![EAST, WEST],
        'L' => vec![NORTH, EAST],
        'J' => vec![NORTH, WEST],
        '7' => vec![SOUTH, WEST],
        'F' => vec![SOUTH, EAST],
        '.' => vec![],
        'S' => vec![NORTH, SOUTH, EAST, WEST],
        _ => unreachable!(),
    }
}

fn is_opposite_direction(first: (isize, isize), second: (isize, isize)) -> bool {
    first.0 == -second.0 && first.1 == -second.1
}

fn main_loop_coordinates(pipes: &[String]) -> HashSet<(usize, usize)> {
    let (mut col, mut row) = start_coordinates(pipes);
    let mut current = first_move(pipes);
    let mut coordinates = HashSet::new();

    loop {
        col = col.checked_add_signed(current.0).unwrap();
        row = row.checked_add_signed(current.1).unwrap();
        coordinates.insert((col, row));

        let pipe = pipes[row].chars().nth(col).unwrap();
        if pipe == START {
            break;
        }

        for next in possible_directions(pipe) {
            if !is_opposite_direction(current, next) {
                current = next;
                break;
            }
        }
    }

    coordinates
}

fn pipes_maze(pipes: &[String], main_loop: &HashSet<(usize, usize)>) -> Vec<Vec<char>> {
    let mut maze = pipes_with_empty_squares(pipes, main_loop);
    let max_x = maze[0].len() - 1;
    let max_y = maze.len() - 1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            let mut character = maze[y][x];

            if x == 0 || x == max_x || y == 0 || y == max_y {
                maze[y][x] = OUTSIDE;
            } else if x % 2 == 1 && y % 2 == 1 {
                if character == '.' {
                    maze[y][x] = START;
                } else {
                    maze[y][x] = '.';

                    if character == START {
                        character = REAL_START;
                    }

                    for (x_forward, y_forward) in possible_directions(character) {
                        let (Some(x_neighbor), Some(y_neighbor)) = (
                            x.checked_add_signed(x_forward),
                            y.checked_add_signed(y_forward),
                        ) else {
                            continue;
                        };

                        maze[y_neighbor][x_neighbor] = '.';
                    }
                }
            }
        }
    }

    maze
}

fn pipes_with_empty_squares(
    pipes: &[String],
    main_loop: &HashSet<(usize, usize)>,
) -> Vec<Vec<char>> {
    let mut extended = Vec::new();
    let new_horizontal = vec![START; pipes[0].len() * 2 + 1];

    extended.push(new_horizontal.clone());
    for (y, line) in pipes.iter().enumerate() {
        let mut new_line = vec![START];

        for (x, character) in line.chars().enumerate() {
            new_line.push(if !main_loop.contains(&(x, y)) {
                '.'
            } else {
                character
            });
            new_line.push(START);
        }

        if new_line.len() != new_horizontal.len() {
            unreachable!()
        }

        extended.push(new_line);
        extended.push(new_horizontal.clone());
    }

    extended
}

fn explore_tiles(
    x_start: usize,
    y_start: usize,
    maze: &[Vec<char>],
    inside_outside: &mut HashMap<(usize, usize), EmptyTileType>,
) {
    let mut queue = VecDeque::new();
    queue.push_back((x_start, y_start));
    let mut verdict = EmptyTileType::Inside;

    while let Some((x, y)) = queue.pop_front() {
        let character = maze[y][x];

        for (x_forward, y_forward) in possible_directions(character) {
            let (Some(x_neighbor), Some(y_neighbor)) = (
                x.checked_add_signed(x_forward),
                y.checked_add_signed(y_forward),
            ) else {
                unreachable!();
            };

            if inside_outside.contains_key(&(x_neighbor, y_neighbor)) {
                continue;
            }

            match maze[y_neighbor][x_neighbor] {
                OUTSIDE => {
                    verdict = EmptyTileType::Outside;
                }
                START => {
                    inside_outside.insert((x_neighbor, y_neighbor), EmptyTileType::Unknown);
                    queue.push_back((x_neighbor, y_neighbor));
                }
                _ => (),
            }
        }
    }

    let tiles: Vec<_> = inside_outside.keys().cloned().collect();
    for tile in tiles {
        if inside_outside.get(&tile).unwrap() == &EmptyTileType::Unknown {
            inside_outside.insert(tile, verdict.clone());
        }
    }
}
