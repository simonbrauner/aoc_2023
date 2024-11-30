use std::collections::HashMap;

pub fn solve(input: &[String]) -> String {
    let reflector = Reflector {
        rows: input
            .iter()
            .map(|line| line.chars().map(Material::new).collect())
            .collect(),
    };

    format!("{}\n{}\n", part_1(reflector.clone()), part_2(reflector))
}

fn part_1(mut reflector: Reflector) -> usize {
    reflector.slide_north();

    reflector.north_load()
}

fn part_2(mut reflector: Reflector) -> usize {
    let mut positions = HashMap::new();

    let mut cycle_counter = 0;
    loop {
        if let Some(cycle_number) = positions.insert(reflector.clone(), cycle_counter) {
            let additional_cycles = (1000000000 - cycle_number) % (cycle_counter - cycle_number);

            for _ in 0..additional_cycles {
                reflector.slide_cycle();
            }

            return reflector.north_load();
        }

        cycle_counter += 1;
        reflector.slide_cycle();
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
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

    fn swap_materials(&mut self, rock_x: usize, rock_y: usize, space_x: usize, space_y: usize) {
        (self.rows[space_y][space_x], self.rows[rock_y][rock_x]) = (
            self.rows[rock_y][rock_x].clone(),
            self.rows[space_y][space_x].clone(),
        );
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
                        self.swap_materials(x, y, x, y - 1);
                        moved = true;
                    }
                }
            }
        }
    }

    fn slide_west(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;

            for x in 1..self.x_size() {
                for y in 0..self.y_size() {
                    if *self.material_at_coords(x, y) == Material::RoundRock
                        && *self.material_at_coords(x - 1, y) == Material::EmptySpace
                    {
                        self.swap_materials(x, y, x - 1, y);
                        moved = true;
                    }
                }
            }
        }
    }

    fn slide_south(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;

            for y in (1..self.y_size()).rev() {
                for x in 0..self.x_size() {
                    if *self.material_at_coords(x, y - 1) == Material::RoundRock
                        && *self.material_at_coords(x, y) == Material::EmptySpace
                    {
                        self.swap_materials(x, y - 1, x, y);
                        moved = true;
                    }
                }
            }
        }
    }

    fn slide_east(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;

            for x in (1..self.x_size()).rev() {
                for y in 0..self.y_size() {
                    if *self.material_at_coords(x - 1, y) == Material::RoundRock
                        && *self.material_at_coords(x, y) == Material::EmptySpace
                    {
                        self.swap_materials(x - 1, y, x, y);
                        moved = true;
                    }
                }
            }
        }
    }

    fn slide_cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
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

#[derive(Clone, PartialEq, Eq, Hash)]
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
