#[derive(PartialEq, Debug)]
pub enum Error {
    FailedToParseId
}

pub fn get_id_sum() -> u32 {
    0
}

fn get_id_sum_from_lines(lines: Vec<String>) -> Result<u32, Error> {
    lines.iter().map(id_from_line).sum()
}

fn id_from_line(line: &String) -> Result<u32, Error> {
    let id_start = line.find(" ").ok_or(Error::FailedToParseId)? + 1;
    let id_end = line.find(":").ok_or(Error::FailedToParseId)?;

    line[id_start..id_end].parse::<u32>().or(Err(Error::FailedToParseId))
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
}