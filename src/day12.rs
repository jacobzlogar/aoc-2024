use std::collections::HashMap;

pub fn run() -> miette::Result<()> {
    let data = crate::get_input("day12")?;
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
    
}

impl Garden {
    fn parse(text: &str) -> miette::Result<Self> {
        Ok(Self {})
    }

    fn solve(&mut self) -> usize {
        1
    }
}
