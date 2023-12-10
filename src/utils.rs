use std::fs::read_to_string;
use std::io::Read;
use std::path::Path;

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum AocError {
    // General Errors
    #[error("Invalid input path")]
    InvalidInputPath,

    // Day 1
    #[error("Failed to parse line")]
    ParseLineFailed,

    // Day 2
    #[error("Failed to parse ID")]
    FailedToParseId,
    #[error("Invalid draw")]
    InvalidDraw,

    // Day 3
    #[error("Unexpected end of stream")]
    UnexpectedEndOfStream
}

pub fn get_lines_from_file(path: &Path) -> Result<Vec<String>, AocError> {
    if let Ok(content) = read_to_string(path.to_str().ok_or(AocError::InvalidInputPath)?) {
        return Ok(content.lines().map(String::from).collect());
    }

    Err(AocError::InvalidInputPath)
}

pub fn read_and_map_line<ReaderT, MapperT, ValueT>(reader: &mut ReaderT, map_byte: MapperT) -> Vec<ValueT>
    where
        ReaderT: std::io::Read,
        MapperT: core::ops::Fn(&u8) -> ValueT,
{
    let mut data = Vec::new();
    let mut buffer = vec![0u8; 1];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 || buffer[0] == b'\n' {
            break;
        }

        data.push(map_byte(&buffer[0]));
    }

    return data;
}

pub fn read_and_map_n<ReaderT, MapperT, ValueT>(reader: &mut ReaderT, map_byte: MapperT, n: usize) -> Result<Vec<ValueT>, AocError>
    where
        ReaderT: std::io::Read,
        MapperT: core::ops::Fn(&u8) -> ValueT,
{
    let mut buffer = vec![0u8; n];

    if reader.read_exact(&mut buffer).is_err() {
        return Err(AocError::UnexpectedEndOfStream);
    }

    Ok(buffer.iter().map(map_byte).collect())
}

pub fn advance_reader<ReaderT: std::io::Read>(reader: &mut ReaderT, n: u64) {
    // We don't care if this fails, since it should only fail on the last line and then we're done anyway.
    let _ = std::io::copy(&mut reader.take(n), &mut std::io::sink());
}