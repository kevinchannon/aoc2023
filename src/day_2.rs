use std::fs::read_to_string;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Error {
    FailedToParseId,
    InvalidInputPaths,
    InvalidGame,
    InvalidDraw
}

#[derive(PartialEq, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

impl Draw {
    pub fn from_string(s: &str) -> Result<Self, Error> {
        let mut out = Self{red: 0, green: 0, blue: 0};

        for colour_count in s.trim().split(",") {
            let parts = colour_count.trim().split(' ').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(Error::InvalidDraw);
            }

            let count = parts[0].parse::<u32>().or(Err(Error::InvalidDraw))?;
            let colour = parts[1];

            match colour {
                "red"   => out.red   = count,
                "green" => out.green = count,
                "blue"  => out.blue  = count,
                _ => return Err(Error::InvalidDraw)
            };
        }

        return Ok(out);
    }
}

pub fn get_id_sum() -> Result<u32, Error> {
    get_id_sum_from_lines(get_lines_from_file(Path::new("inputs/day2.txt"))?)
}

fn get_lines_from_file(path: &Path) -> Result<Vec<String>, Error> {
    if let Ok(content) = read_to_string(path.to_str().ok_or(Error::InvalidInputPaths)?) {
        return Ok(content.lines().map(String::from).collect());
    }

    Err(Error::InvalidInputPaths)
}

fn get_id_sum_from_lines(lines: Vec<String>) -> Result<u32, Error> {
    lines.iter().filter(|l| match valid_game(&l) { Ok(true) => true, _ => false} ).map(id_from_line).sum()
}

fn id_from_line(line: &String) -> Result<u32, Error> {
    let id_start = line.find(" ").ok_or(Error::FailedToParseId)? + 1;
    let id_end = line.find(":").ok_or(Error::FailedToParseId)?;

    line[id_start..id_end].parse::<u32>().or(Err(Error::FailedToParseId))
}

fn valid_game(line: &&String) -> Result<bool, Error> {
    if let Some(pos) = line.find(":") {
        return Ok(line[pos + 1..].split(";").fold(true,
                                           |x, d| x && match draw_is_valid(d) {
                                               Ok(true) => true,
                                               _ => false
                                           }));
    } else {
        return Err(Error::InvalidGame);
    }
}

fn draw_is_valid(draw: &str) -> Result<bool, Error> {
    for colour_count in draw.trim().split(",") {
        let parts = colour_count.trim().split(' ').collect::<Vec<&str>>();
        let count = parts[0].parse::<u32>().or(Err(Error::InvalidDraw))?;
        let colour = parts[1];

        if !colour_count_is_valid(colour, count) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn colour_count_is_valid(colour: &str, count: u32) -> bool {
    match colour {
        "red" if count <= 12 => true,
        "green" if count <= 13 => true,
        "blue" if count <= 14 => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_sum_returns_id_of_single_possible_game() {
        assert_eq!(Ok(1), get_id_sum_from_lines(vec![String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]));
    }

    #[test]
    fn get_id_sum_returns_id_of_multiple_possible_games() {
        let lines = vec![String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                         String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")];
        assert_eq!(Ok(3), get_id_sum_from_lines(lines));
    }

    #[test]
    fn draw_is_valid_returns_true_for_valid_draw() {
        assert_eq!(Ok(true), draw_is_valid(" 12 red"));
        assert_eq!(Ok(true), draw_is_valid(" 13 green"));
        assert_eq!(Ok(true), draw_is_valid(" 14 blue"));
        assert_eq!(Ok(true), draw_is_valid(" 12 red, 13 green, 14 blue"));
    }

    #[test]
    fn draw_is_valid_returns_false_for_invalid_draw() {
        assert_eq!(Ok(false), draw_is_valid(" 13 red"));
        assert_eq!(Ok(false), draw_is_valid(" 14 green"));
        assert_eq!(Ok(false), draw_is_valid(" 15 blue"));
        assert_eq!(Ok(false), draw_is_valid(" 13 red, 13 green, 14 blue"));
        assert_eq!(Ok(false), draw_is_valid(" 13 red, 14 green, 14 blue"));
        assert_eq!(Ok(false), draw_is_valid(" 13 red, 13 green, 15 blue"));
    }

    #[test]
    fn valid_game_is_false_for_invalid_game() {
        let game = String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(Ok(false), valid_game(&&game));
    }

    #[test]
    fn example_games_work() {
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        ];

        assert_eq!(Ok(8), get_id_sum_from_lines(lines));
    }

    #[test]
    fn create_draw_from_string() {
        assert_eq!(Ok(Draw{red: 1, green: 2, blue: 3}), Draw::from_string(" 2 green, 3 blue, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_for_invalid_colour() {
        assert_eq!(Err(Error::InvalidDraw), Draw::from_string(" 2 green, 3 yellow, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_for_invalid_value() {
        assert_eq!(Err(Error::InvalidDraw), Draw::from_string(" 2 green, 3 blue, ? red"));
    }

    #[test]
    fn create_draw_from_string_fails_if_there_are_too_few_parts() {
        assert_eq!(Err(Error::InvalidDraw), Draw::from_string(" 2 , 3 yellow, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_if_there_are_too_many_parts() {
        assert_eq!(Err(Error::InvalidDraw), Draw::from_string(" 2 green X, 3 yellow, 1 red"));
    }
}