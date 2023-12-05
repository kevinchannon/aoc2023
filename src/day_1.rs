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
    let numbers = [
        ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
        ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),
        ("0", "0"), ("1", "1"), ("2", "2"), ("3", "3"), ("4", "4"),
        ("5", "5"), ("6", "6"), ("7", "7"), ("8", "8"), ("9", "9")
    ];

    let mut out = String::new();

    for idx in 0..s.len() {
        for n in numbers {
            if s[idx..].starts_with(n.0) {
                out += n.1;
            }
        }
    }

    out
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
    fn lines_with_word_number_examples() {
        assert_eq!(Ok(29), int_from_line(words_to_digit_chars(&String::from("two1nine"))));
        assert_eq!(Ok(83), int_from_line(words_to_digit_chars(&String::from("eightwothree"))));
        assert_eq!(Ok(13), int_from_line(words_to_digit_chars(&String::from("abcone2threexyz"))));
        assert_eq!(Ok(24), int_from_line(words_to_digit_chars(&String::from("xtwone3four"))));
        assert_eq!(Ok(42), int_from_line(words_to_digit_chars(&String::from("4nineeightseven2"))));
        assert_eq!(Ok(14), int_from_line(words_to_digit_chars(&String::from("zoneight234"))));
        assert_eq!(Ok(76), int_from_line(words_to_digit_chars(&String::from("7pqrstsixteen"))));
    }

    #[test]
    fn words_to_digit_chars_replaces_digit() {
        assert_eq!("0".to_string(), words_to_digit_chars(&"zero".to_string()));
        assert_eq!("1".to_string(), words_to_digit_chars(&"one".to_string()));
        assert_eq!("2".to_string(), words_to_digit_chars(&"two".to_string()));
        assert_eq!("3".to_string(), words_to_digit_chars(&"three".to_string()));
        assert_eq!("4".to_string(), words_to_digit_chars(&"four".to_string()));
        assert_eq!("5".to_string(), words_to_digit_chars(&"five".to_string()));
        assert_eq!("6".to_string(), words_to_digit_chars(&"six".to_string()));
        assert_eq!("7".to_string(), words_to_digit_chars(&"seven".to_string()));
        assert_eq!("8".to_string(), words_to_digit_chars(&"eight".to_string()));
        assert_eq!("9".to_string(), words_to_digit_chars(&"nine".to_string()));
    }

    #[test]
    fn words_to_digit_chars_replaces_multiple_digits() {
        assert_eq!("823".to_string(), words_to_digit_chars(&"eighttwothree".to_string()));
    }

    #[test]
    fn words_to_digit_chars_overlapping_words() {
        assert_eq!("01".to_string(), words_to_digit_chars(&String::from("zerone")));
        assert_eq!("18".to_string(), words_to_digit_chars(&String::from("oneight")));
        assert_eq!("21".to_string(), words_to_digit_chars(&String::from("twone")));
        assert_eq!("38".to_string(), words_to_digit_chars(&String::from("threeight")));
        assert_eq!("58".to_string(), words_to_digit_chars(&String::from("fiveight")));
        assert_eq!("79".to_string(), words_to_digit_chars(&String::from("sevenine")));
        assert_eq!("82".to_string(), words_to_digit_chars(&String::from("eightwo")));
        assert_eq!("83".to_string(), words_to_digit_chars(&String::from("eighthree")));
        assert_eq!("98".to_string(), words_to_digit_chars(&String::from("nineight")));
    }
}