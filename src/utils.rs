use std::fs::read_to_string;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum Error {
    // General Errors
    InvalidInputPath,

    // Day 1
    ParseLineFailed,

    // Day 2
    FailedToParseId,
    InvalidInputPaths,
    InvalidDraw,
}

pub fn get_lines_from_file(path: &Path) -> Result<Vec<String>, Error> {
    if let Ok(content) = read_to_string(path.to_str().ok_or(Error::InvalidInputPath)?) {
        return Ok(content.lines().map(String::from).collect());
    }

    Err(Error::InvalidInputPath)
}
