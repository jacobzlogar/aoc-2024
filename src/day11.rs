use std::collections::HashMap;

pub fn run() -> miette::Result<()> {
    let data = crate::get_input("day11")?;
    println!("Day 11, part 1: {}", part_1(&data, 25)?);
    println!("Day 11, part 2: {}", part_2(&data, 75)?);
    Ok(())
}

#[tracing::instrument(skip_all)]
fn part_1(data: &str, steps: u8) -> miette::Result<usize> {
    let mut arrangement = Arrangement::parse(data)?;
    Ok(arrangement.solve(steps))
}

#[tracing::instrument(skip_all)]
fn part_2(data: &str, steps: u8) -> miette::Result<usize> {
    let mut arrangement = Arrangement::parse(data)?;
    Ok(arrangement.solve(steps))
}

#[derive(Debug, Clone)]
pub struct Arrangement {
    frequency: HashMap<usize, usize>,
    result: usize,
}

impl Arrangement {
    fn parse(text: &str) -> miette::Result<Self> {
        let stones: miette::Result<Vec<usize>> = text
            .split_whitespace()
            .map(|stone| {
                stone
                    .parse::<usize>()
                    .map_err(|e| miette::miette!("{e} {stone}"))
            })
            .collect();
        let stones = stones?;
        let frequency = HashMap::new();
        let mut arrangement = Arrangement {
            frequency,
            result: 0,
        };
        for stone in stones.clone() {
            arrangement.frequency.entry(stone).and_modify(|e| *e += 1)
                .or_insert(1);
        }
        Ok(arrangement)
    }

    fn solve(&mut self, steps: u8) -> usize {
        for _ in 0..steps {
            let mut frequency = HashMap::new();

            for (&stone, &count) in &self.frequency {
                if stone == 0 {
                    *frequency.entry(1).or_insert(0) += count;
                    continue;
                }

                let len = (stone as f64).log10().floor() as usize + 1;
                if len % 2 == 0 {
                    let (left, right) = split_digit(&stone, len);
                    *frequency.entry(left).or_insert(0) += count;
                    *frequency.entry(right).or_insert(0) += count;
                } else {
                    *frequency.entry(stone * 2024).or_insert(0) += count;
                }
            }

            self.frequency = frequency;
        }

        self.frequency.values().sum()
    }
}

fn split_digit(stone: &usize, len: usize) -> (usize, usize) {
    let digit_string = stone.to_string();
    let (left, right) = digit_string.split_at(len / 2);
    let left_digit = left.parse::<usize>().unwrap();
    let right_digit = right.trim_start_matches('0').parse::<usize>().unwrap_or(0);
    (left_digit, right_digit)
}

#[cfg(test)]
#[test]
fn test_day11_part1() {
    let test = "125 17";
    let mut arrangement = Arrangement::parse(test).unwrap();
    assert_eq!(arrangement.solve(6), 22);
    let mut arrangement = Arrangement::parse(test).unwrap();
    assert_eq!(arrangement.solve(25), 55312);
}
