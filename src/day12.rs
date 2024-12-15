use std::collections::HashMap;

#[derive(Debug)]
pub struct Garden {
    regions: HashMap<char, Vec<(usize, usize)>>,
}

impl Garden {
    pub fn parse(text: &str) -> miette::Result<Self> {
        let mut regions = HashMap::new();
        for (row_index, row) in text.lines().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                regions
                    .entry(col)
                    .and_modify(|e: &mut Vec<(usize, usize)>| e.push((row_index, col_index)))
                    .or_insert(vec![(row_index, col_index)]);
            }
        }
        Ok(Self { regions })
    }

    pub fn solve(&mut self) -> usize {
        1
    }
}

const TEST: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

#[cfg(test)]
mod tests {
    use super::{TEST, Garden};

    #[test]
    fn test_part1() {
        let garden = Garden::parse(TEST).unwrap();
        println!("{:?}", garden);
    }
}
