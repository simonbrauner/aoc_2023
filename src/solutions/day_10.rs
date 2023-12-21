pub fn solve(input: &[String]) -> String {
    format!("{}\n{}\n", part_1(input), part_2())
}

fn part_1(pipes: &[String]) -> isize {
    let (mut col, mut row) = start_coordinates(pipes);
    let mut current = first_move(pipes);
    let mut steps: isize = 0;

    loop {
        col = col.checked_add_signed(current.0).unwrap();
        row = row.checked_add_signed(current.1).unwrap();
        steps += 1;

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

    steps / 2
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
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
