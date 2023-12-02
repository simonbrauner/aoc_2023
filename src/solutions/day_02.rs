use regex::Regex;

pub fn solve(input: &[String]) -> String {
    let id_re = Regex::new(r"Game (\d+)").unwrap();
    let cube_re = Regex::new(r"(\d+) ((?:red)|(?:green)|(?:blue))").unwrap();
    let mut games = Vec::new();

    for line in input {
        let id = id_re.captures(line).unwrap()[1].parse().unwrap();
        let mut cubes = Vec::new();

        for (_, [count, color]) in cube_re.captures_iter(line).map(|c| c.extract()) {
            cubes.push((
                count.parse().unwrap(),
                match color {
                    "red" => CubeColor::Red,
                    "green" => CubeColor::Green,
                    "blue" => CubeColor::Blue,
                    _ => panic!(),
                },
            ));
        }

        games.push(Game { id, cubes });
    }

    format!("{}\n{}\n", part_1(&games), part_2())
}

fn part_1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            if game.is_possible() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part_2() -> String {
    "part 2 unimplemented".to_string()
}

struct Game {
    id: u32,
    cubes: Vec<(u32, CubeColor)>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.cubes
            .iter()
            .all(|(count, color)| count <= &color.maximal_count())
    }
}

enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn maximal_count(&self) -> u32 {
        match self {
            CubeColor::Red => 12,
            CubeColor::Green => 13,
            CubeColor::Blue => 14,
        }
    }
}
