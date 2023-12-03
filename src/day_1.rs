use std::fs::read_to_string;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Error {
    InvalidInputPaths,
    ParseLineFailed
}

pub fn get_calibration_factor() -> Result<u32, Error> {
    get_lines_from_file(Path::new("inputs/day1_part1.txt"))?
        .iter()
        .map(words_to_digit_chars)
        .map(int_from_line)
        .sum()
}

fn get_lines_from_file(path: &Path) -> Result<Vec<String>, Error> {
    if let Ok(content) = read_to_string(path.to_str().ok_or(Error::InvalidInputPaths)?) {
        return Ok(content.lines().map(String::from).collect());
    }

    Err(Error::InvalidInputPaths)
}

fn words_to_digit_chars(s: &String) -> String {
    s.replace("one", "1")
}

fn int_from_line(line: String) -> Result<u32, Error> {
    let ten = line.chars()
        .find(|c| c.is_digit(10))
        .ok_or(Error::ParseLineFailed)?
        .to_digit(10)
        .expect("We only selected base-10 digits in the find, so we shouldn't ever get here");
    let unit = line.chars()
        .rfind(|c| c.is_digit(10))
        .ok_or(Error::ParseLineFailed)?
        .to_digit(10)
        .expect("We only selected base-10 digits in the find, so we shouldn't ever get here");

    Ok(10 * ten + unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_from_line_works_on_trivial_line() {
        assert_eq!(Ok(1), int_from_line(String::from("01")));
    }

    #[test]
    fn int_from_line_works_on_line_with_letters() {
        assert_eq!(Ok(42), int_from_line(String::from("4a2")));
        assert_eq!(Ok(42), int_from_line(String::from("a42")));
        assert_eq!(Ok(42), int_from_line(String::from("42a")));
        assert_eq!(Ok(42), int_from_line(String::from("a4b2")));
        assert_eq!(Ok(42), int_from_line(String::from("a42b")));
        assert_eq!(Ok(42), int_from_line(String::from("4a2b")));
    }

    #[test]
    fn int_from_line_works_on_line_with_multiple_numbers() {
        assert_eq!(Ok(42), int_from_line(String::from("412")));
    }

    #[test]
    fn int_from_line_works_on_line_with_single() {
        assert_eq!(Ok(44), int_from_line(String::from("4")));
    }

    #[test]
    fn int_from_line_fails_with_invalid_parse_error_when_there_are_no_numbers() {
        assert_eq!(Err(Error::ParseLineFailed), int_from_line(String::from("a")));
    }

    #[test]
    fn int_from_line_works_for_example_lines() {
        assert_eq!(Ok(12), int_from_line(String::from("1abc2")));
        assert_eq!(Ok(38), int_from_line(String::from("pqr3stu8vwx")));
        assert_eq!(Ok(15), int_from_line(String::from("a1b2c3d4e5f")));
        assert_eq!(Ok(77), int_from_line(String::from("treb7uchet")));
    }

    #[test]
    fn words_to_digit_chars_replaces_digit() {
        assert_eq!("1".to_string(), words_to_digit_chars(&"one".to_string()));
    }
}