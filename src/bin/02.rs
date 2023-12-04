#[macro_use]
extern crate lazy_static;

use regex::Regex;
advent_of_code::solution!(2);

lazy_static! {
    pub static ref CUBE_COUNT_RE: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

impl Cube {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(Cube::Red),
            "green" => Some(Cube::Green),
            "blue" => Some(Cube::Blue),
            _ => None,
        }
    }
}

/// No, not GameCube™, but the maximum count of each colored cube across
/// all subsets within a unique game.
#[derive(Default, Debug)]
struct GameCubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameCubeCounts {
    fn update_max_count(&mut self, color: Cube, count: u32) {
        match color {
            Cube::Red => self.red = self.red.max(count),
            Cube::Green => self.green = self.green.max(count),
            Cube::Blue => self.blue = self.blue.max(count),
        }
    }

    /// Determines if the cubes revealed for each color does not exceed the available cubes
    fn is_game_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    /// The fewest cubes required to satisfy a game is the power of the count of each cube per color
    fn fewest_cubes_required(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

/// Calculates the max number of cubes revealed per color across all subsets within a given game.
fn count_max_cubes_revealed(input: &str) -> GameCubeCounts {
    let mut counts = GameCubeCounts::default();

    for capture in CUBE_COUNT_RE.captures_iter(input) {
        if let (Some(count_match), Some(color_match)) = (capture.get(1), capture.get(2)) {
            let count = count_match.as_str().parse::<u32>().unwrap_or(0);
            if let Some(color) = Cube::from_str(color_match.as_str()) {
                counts.update_max_count(color, count);
            }
        }
    }

    counts
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, game)| {
            let counts = count_max_cubes_revealed(game);
            if counts.is_game_possible() {
                Some(index as u32 + 1)
            } else {
                None
            }
        })
        .sum::<u32>()
        .into()
}
pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|game| count_max_cubes_revealed(game).fewest_cubes_required())
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
