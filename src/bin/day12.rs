use std::collections::HashMap;

pub fn main() -> miette::Result<()> {
    let data = aoc_2024::get_input("day12")?;
    println!("Day 12, part 1: {}", part_1(&data)?);
    println!("Day 12, part 2: {}", part_2(&data)?);
    Ok(())
}

#[tracing::instrument(skip_all)]
fn part_1(data: &str) -> miette::Result<usize> {
    let mut arrangement = Garden::parse(data)?;
    Ok(arrangement.solve())
}

#[tracing::instrument(skip_all)]
fn part_2(data: &str) -> miette::Result<usize> {
    let mut arrangement = Garden::parse(data)?;
    Ok(arrangement.solve())
}

struct Garden {
    regions: HashMap<char, Vec<(usize, usize)>>
}

impl Garden {
    fn parse(text: &str) -> miette::Result<Self> {
        let mut regions = HashMap::new();
        for (row_index, row) in text.lines().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                regions.entry(col)
                    .and_modify(|e: &mut Vec<(usize, usize)>| e.push((row_index, col_index)))
                    .or_insert(vec![(row_index, col_index)]);
            }
        }
        Ok(Self { regions })
    }

    fn solve(&mut self) -> usize {
        1
    }
}

const TEST: &str =
r#"AAAA
BBCD
BBCC
EEEC"#;

#[cfg(test)]
#[test]
fn test_day12_part1() {
}
