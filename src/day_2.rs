use std::path::Path;
use crate::utils::{AocError, get_lines_from_file};

#[derive(PartialEq, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

impl Draw {
    pub fn from_string(s: &str) -> Result<Self, AocError> {
        let mut out = Self{red: 0, green: 0, blue: 0};

        for colour_count in s.trim().split(",") {
            let (count, colour) = Draw::parse_for_count_and_colour(colour_count)?;

            match colour {
                "red"   => out.red   = count,
                "green" => out.green = count,
                "blue"  => out.blue  = count,
                _ => return Err(AocError::InvalidDraw)
            };
        }

        return Ok(out);
    }

    fn parse_for_count_and_colour(s: &str) -> Result<(u32, &str), AocError> {
        let parts = s.trim().split(' ').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(AocError::InvalidDraw);
        }

        Ok((parts[0].parse::<u32>().or(Err(AocError::InvalidDraw))?, parts[1]))
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>
}

impl Game {
    pub fn from_string(s: &str) -> Result<Self, AocError> {
        let (id, draws) = Game::get_id_and_draws_str(s)?;
        let out = Game{id, draws: Game::parse_draws(draws)?};

        Ok(out)
    }

    pub fn is_possible(self: &Self) -> bool {
        self.draws.iter().all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14 )
    }

    pub fn power(self: &Self) -> u32 {
        let accumulate_min_colour_values = |mins: [u32; 3], d: &Draw| {
            [std::cmp::max(mins[0], d.red),
                std::cmp::max(mins[1], d.green),
                std::cmp::max(mins[2], d.blue)]
        };

        let product = |prod, x| prod * x;

        self.draws.iter()
            .fold([0; 3], accumulate_min_colour_values)
            .iter()
            .fold(1, product)
    }

    fn get_id_and_draws_str(s: &str) -> Result<(u32, &str), AocError> {
        let id_start = s.find(" ").ok_or(AocError::FailedToParseId)? + 1;
        let id_end = s.find(":").ok_or(AocError::FailedToParseId)?;

        if let Ok(id) = s[id_start..id_end].parse::<u32>() {
            return Ok((id, &s[id_end + 1..]));
        }

        Err(AocError::FailedToParseId)
    }

    fn parse_draws(s: &str) -> Result<Vec<Draw>, AocError> {
        let mut out = Vec::<Draw>::new();
        for draw_str in s.split(";") {
            out.push(Draw::from_string(draw_str)?);
        }

        Ok(out)
    }
}

pub fn get_id_sum() -> Result<u32, AocError> {
    get_id_sum_from_lines(&get_lines_from_file(Path::new("inputs/day2.txt"))?)
}

pub fn get_total_power() -> Result<u32, AocError> {
    Ok(games_from_lines(&get_lines_from_file(Path::new("inputs/day2.txt"))?)?.iter()
        .map(|g| g.power())
        .sum())
}

fn games_from_lines(lines: &Vec<String>) -> Result<Vec<Game>, AocError> {
    lines.iter()
        .map( |l| { Game::from_string(l.as_str())})
        .collect()
}

fn get_id_sum_from_lines(lines: &Vec<String>) -> Result<u32, AocError> {
    Ok(games_from_lines(lines)?.iter()
        .filter(|g| g.is_possible() )
        .map(|g| g.id)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_games_work() {
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        ];

        assert_eq!(Ok(8), get_id_sum_from_lines(&lines));
    }

    #[test]
    fn create_draw_from_string() {
        assert_eq!(Ok(Draw{red: 1, green: 2, blue: 3}), Draw::from_string(" 2 green, 3 blue, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_for_invalid_colour() {
        assert_eq!(Err(AocError::InvalidDraw), Draw::from_string(" 2 green, 3 yellow, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_for_invalid_value() {
        assert_eq!(Err(AocError::InvalidDraw), Draw::from_string(" 2 green, 3 blue, ? red"));
    }

    #[test]
    fn create_draw_from_string_fails_if_there_are_too_few_parts() {
        assert_eq!(Err(AocError::InvalidDraw), Draw::from_string(" 2 , 3 yellow, 1 red"));
    }

    #[test]
    fn create_draw_from_string_fails_if_there_are_too_many_parts() {
        assert_eq!(Err(AocError::InvalidDraw), Draw::from_string(" 2 green X, 3 yellow, 1 red"));
    }

    #[test]
    fn create_game_from_string() {
        let expected = Game{id: 1, draws: vec![Draw{red: 4, green: 0, blue: 3}, Draw{red: 1, green: 2, blue: 6}, Draw{red: 0, green: 2, blue: 0}]};
        assert_eq!(Ok(expected), Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn create_game_from_string_fails_if_id_is_bad() {
        assert_eq!(Err(AocError::FailedToParseId), Game::from_string("Game ?: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn create_game_from_string_fails_if_draws_are_bad() {
        assert_eq!(Err(AocError::InvalidDraw), Game::from_string("Game 2: wibble"));
    }

    #[test]
    fn games_from_lines_works_for_good_games() {
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        ];

        let games = games_from_lines(&lines);
        assert!(games.is_ok());
    }

    #[test]
    fn games_from_lines_returns_invalid_game_when_it_fails_to_parse() {
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 CHEESE!, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        ];

        let games = games_from_lines(&lines);
        assert_eq!(Err(AocError::InvalidDraw), games);
    }

    #[test]
    fn part_1_result_is_correct() {
        assert_eq!(Ok(2237), get_id_sum());
    }
    #[test]
    fn part_2_result_is_correct() {
        assert_eq!(Ok(66681), get_total_power());
    }
}