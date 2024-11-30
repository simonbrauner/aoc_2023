pub fn solve(input: &[String]) -> String {
    let mut patterns = vec![Pattern::new()];

    for line in input {
        if line.is_empty() {
            patterns.push(Pattern::new());
        } else {
            let material_row: Vec<_> = line.chars().map(Material::new).collect();
            patterns.last_mut().unwrap().add_row(material_row);
        }
    }

    format!("{}\n{}\n", part_1(&patterns), part_2())
}

fn part_1(patterns: &Vec<Pattern>) -> usize {
    let mut summarization = 0;

    for pattern in patterns {
        if let Some(column_count) = pattern.left_column_count() {
            summarization += column_count;
            continue;
        }

        if let Some(column_count) = pattern.up_column_count() {
            summarization += 100 * column_count;
            continue;
        }

        unreachable!();
    }

    summarization
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Pattern {
    rows: Vec<Vec<Material>>,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern { rows: Vec::new() }
    }

    fn add_row(&mut self, row: Vec<Material>) {
        self.rows.push(row);
    }

    fn x_size(&self) -> usize {
        self.rows[0].len()
    }

    fn y_size(&self) -> usize {
        self.rows.len()
    }

    fn material_at_coords(&self, x: usize, y: usize) -> &Material {
        &self.rows[y][x]
    }

    fn left_column_count(&self) -> Option<usize> {
        'outer: for x in 1..self.x_size() {
            let mut x_up = x - 1;
            let mut x_down = x;

            loop {
                for y in 0..self.y_size() {
                    if self.material_at_coords(x_up, y) != self.material_at_coords(x_down, y) {
                        continue 'outer;
                    }
                }

                if x_up == 0 || x_down == self.x_size() - 1 {
                    return Some(x);
                }

                x_up -= 1;
                x_down += 1;
            }
        }

        None
    }

    fn up_column_count(&self) -> Option<usize> {
        'outer: for y in 1..self.y_size() {
            let mut y_left = y - 1;
            let mut y_right = y;

            loop {
                for x in 0..self.x_size() {
                    if self.material_at_coords(x, y_left) != self.material_at_coords(x, y_right) {
                        continue 'outer;
                    }
                }

                if y_left == 0 || y_right == self.rows.len() - 1 {
                    return Some(y);
                }

                y_left -= 1;
                y_right += 1;
            }
        }

        None
    }
}

#[derive(PartialEq)]
enum Material {
    Ash,
    Rocks,
}

impl Material {
    fn new(ch: char) -> Material {
        match ch {
            '.' => Material::Ash,
            '#' => Material::Rocks,
            _ => unreachable!(),
        }
    }
}
