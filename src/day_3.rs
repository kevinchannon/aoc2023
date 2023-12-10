use crate::utils;
use crate::utils::AocError;

#[derive(PartialEq, Debug)]
enum CellValue {
    Space,
    Symbol,
    Digit(char),
}

impl CellValue {
    pub fn from_byte(b: &u8) -> Self {
        match b {
            b'.' => CellValue::Space,
            b'0'..=b'9' => CellValue::Digit(*b as char),
            _ => CellValue::Symbol
        }
    }
}

#[derive(PartialEq, Debug)]
struct Grid {
    data: Vec<CellValue>,
    width: usize,
}

impl Grid {
    pub fn from_stream<R: std::io::Read>(reader: &mut R) -> Self {
        let mut data = utils::read_and_map_line(reader, CellValue::from_byte);
        let width = data.len();

        while let Ok(mut row) = utils::read_and_map_n(reader, CellValue::from_byte, width) {
            data.append(&mut row);

            // Consume the newline character
            utils::advance_reader(reader, 1);
        }

        Self { data, width }
    }

    #[allow(dead_code)]
    pub fn at(self: &Self, x: usize, y: usize) -> &CellValue {
        &self.data[y * self.width + x]
    }
}

pub fn part_number_sum<ReaderT: std::io::Read>(reader: &mut ReaderT) -> Result<u32, AocError> {
    let _grid = Grid::from_stream(reader);

    Ok(0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn read_single_row_grid() {
        let input = String::from(".#/?0123456789..");
        let expected_width = input.len();
        let mut cursor = std::io::Cursor::new(input);

        let expected_data = vec![
            CellValue::Space,
            CellValue::Symbol,
            CellValue::Symbol,
            CellValue::Symbol,
            CellValue::Digit('0'),
            CellValue::Digit('1'),
            CellValue::Digit('2'),
            CellValue::Digit('3'),
            CellValue::Digit('4'),
            CellValue::Digit('5'),
            CellValue::Digit('6'),
            CellValue::Digit('7'),
            CellValue::Digit('8'),
            CellValue::Digit('9'),
            CellValue::Space,
            CellValue::Space];

        assert_eq!(Grid { data: expected_data, width: expected_width }, Grid::from_stream(&mut cursor));
    }

    #[test]
    fn can_read_eaxample_grid() {
        let input = String::from(indoc!{
           "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."});
        let mut reader = std::io::Cursor::new(input.clone());

        let grid = Grid::from_stream(&mut reader);
        assert_eq!(10, grid.width);
        for y in 0..grid.width {
            for x in 0..grid.width {
                let byte = input.as_str().as_bytes()[y * (grid.width + 1) + x];
                let expected = match byte {
                    b'.' => CellValue::Space,
                    b'0'..=b'9' => CellValue::Digit(byte as char),
                    _ => CellValue::Symbol,
                };

                assert_eq!(&expected, grid.at(x, y));
            }
        }
    }
}
