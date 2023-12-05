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
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];

    let mut out = s.clone();

    for idx in 0..out.len() {
        if idx >= out.len() {
            break;
        }

        for n in numbers {
            if out[idx..].starts_with(n) {
                out = replace_number_word_at(out, idx);
                break;
            }
        }
    }

    out
}

fn replace_number_word_at(s: String, idx: usize) -> String {
    match &s[idx..] {
        sub if sub.starts_with("zero") => s.replace("zero", "0"),
        sub if sub.starts_with("one") => s.replace("one", "1"),
        sub if sub.starts_with("two") => s.replace("two", "2"),
        sub if sub.starts_with("three") => s.replace("three", "3"),
        sub if sub.starts_with("four") => s.replace("four", "4"),
        sub if sub.starts_with("five") => s.replace("five", "5"),
        sub if sub.starts_with("six") => s.replace("six", "6"),
        sub if sub.starts_with("seven") => s.replace("seven", "7"),
        sub if sub.starts_with("eight") => s.replace("eight", "8"),
        sub if sub.starts_with("nine") => s.replace("nine", "9"),
        _ => s
    }
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
        assert_eq!("0ne".to_string(), words_to_digit_chars(&String::from("zerone")));
        assert_eq!("1ight".to_string(), words_to_digit_chars(&String::from("oneight")));
        assert_eq!("2ne".to_string(), words_to_digit_chars(&String::from("twone")));
        assert_eq!("3ight".to_string(), words_to_digit_chars(&String::from("threeight")));
        assert_eq!("5ight".to_string(), words_to_digit_chars(&String::from("fiveight")));
        assert_eq!("7ine".to_string(), words_to_digit_chars(&String::from("sevenine")));
        assert_eq!("8wo".to_string(), words_to_digit_chars(&String::from("eightwo")));
        assert_eq!("8hree".to_string(), words_to_digit_chars(&String::from("eighthree")));
        assert_eq!("9ight".to_string(), words_to_digit_chars(&String::from("nineight")));
    }
}