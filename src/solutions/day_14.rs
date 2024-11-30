pub fn solve(input: &[String]) -> String {
    let reflector = Reflector {
        rows: input
            .iter()
            .map(|line| line.chars().map(Material::new).collect())
            .collect(),
    };

    format!("{}\n{}\n", part_1(reflector.clone()), part_2())
}

fn part_1(mut reflector: Reflector) -> usize {
    reflector.slide_north();

    reflector.north_load()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

#[derive(Clone)]
struct Reflector {
    rows: Vec<Vec<Material>>,
}

impl Reflector {
    fn x_size(&self) -> usize {
        self.rows[0].len()
    }

    fn y_size(&self) -> usize {
        self.rows.len()
    }

    fn material_at_coords(&self, x: usize, y: usize) -> &Material {
        &self.rows[y][x]
    }

    fn slide_rock(&mut self, rock_x: usize, rock_y: usize, space_x: usize, space_y: usize) {
        self.rows[space_y][space_x] = Material::RoundRock;
        self.rows[rock_y][rock_x] = Material::EmptySpace;
    }

    fn slide_north(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;

            for y in 1..self.y_size() {
                for x in 0..self.x_size() {
                    if *self.material_at_coords(x, y) == Material::RoundRock
                        && *self.material_at_coords(x, y - 1) == Material::EmptySpace
                    {
                        self.slide_rock(x, y, x, y - 1);
                        moved = true;
                    }
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        let mut load = 0;

        let mut rows_to_south_edge = self.y_size();
        for y in 0..self.y_size() {
            for x in 0..self.x_size() {
                if *self.material_at_coords(x, y) == Material::RoundRock {
                    load += rows_to_south_edge;
                }
            }

            rows_to_south_edge -= 1;
        }

        load
    }
}

#[derive(Clone, PartialEq)]
enum Material {
    RoundRock,
    CubeRock,
    EmptySpace,
}

impl Material {
    fn new(ch: char) -> Material {
        match ch {
            'O' => Material::RoundRock,
            '#' => Material::CubeRock,
            '.' => Material::EmptySpace,
            _ => unreachable!(),
        }
    }
}
