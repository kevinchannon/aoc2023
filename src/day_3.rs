use crate::utils::Error;

struct Grid {
    data: Vec<String>
}

impl Grid {
    pub fn from(lines: Vec<String>) -> Self {
        Self{data: lines}
    }

    pub fn at(self: &Self, x: usize, y: usize) -> Option<char> {
        self.data[y].chars().nth(x)
    }
}

pub fn part_number_sum(input: Vec<String>) -> Result<u32, Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_2_by_2_case() {
        let input = vec![String::from("1."),
                         String::from(".#")];

        // assert_eq!(Ok(1), part_number_sum(input));
    }
}
