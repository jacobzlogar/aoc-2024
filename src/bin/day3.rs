use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Cond {
    Enabled,
    Disabled,
}

impl FromStr for Cond {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "do()" => Ok(Self::Enabled),
            "don't()" => Ok(Self::Disabled),
            _ => Err("Not a cond".into()),
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("data/day3.txt")?;
    let iter = data.chars();
    let part_1: i32 = parse(&mut iter.clone(), false)?.iter().sum();
    println!("Day 3, part 1: {:?}", part_1);
    let part_2: i32 = parse(&mut iter.clone(), true)?.iter().sum();
    println!("Day 3, part 2: {:?}", part_2);
    Ok(())
}

fn parse<'a, I: Iterator<Item = char>>(
    iter: &'a mut I,
    conditional: bool,
) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut operations = vec![];
    let mut enabled = Cond::Enabled;
    while let Some(ch) = iter.next() {
        if ch == 'm' || ch == 'd' {
            let mut arith: Vec<char> = vec![ch];
            while let Some(c) = iter.next() {
                arith.push(c);
                if c == 'm' || c == 'd' {
                    arith = vec![c];
                }
                if c == ')' {
                    let text = String::from_iter(arith.clone());
                    if let Ok((_, cond)) = parse_conditionals(&text) {
                        enabled = Cond::from_str(cond)?;
                    }
                    if let Ok((_, digits)) = parse_mul(&text) {
                        if conditional && enabled == Cond::Enabled {
                            operations.push(digits.0 * digits.1);
                        }
                        if !conditional {
                            operations.push(digits.0 * digits.1);
                        }
                    }
                    break;
                }
            }
        }
    }
    Ok(operations)
}

fn parse_operands(text: &str) -> IResult<&str, (i32, i32)> {
    delimited(tag("("), separated_pair(i32, tag(","), i32), tag(")"))(text)
}

fn parse_mul(text: &str) -> IResult<&str, (i32, i32)> {
    let (rest, _) = tag("mul")(text)?;
    parse_operands(rest)
}

fn parse_do(text: &str) -> IResult<&str, &str> {
    tag("do()")(text)
}

fn parse_dont(text: &str) -> IResult<&str, &str> {
    tag("don't()")(text)
}

fn parse_conditionals(text: &str) -> IResult<&str, &str> {
    alt((parse_dont, parse_do))(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_mul() {
        let input = "mul(12,32)";
        assert_eq!(parse_mul(input), Ok(("", (12, 32))));
    }
    #[test]
    fn test_parse_operands() {
        let input = "(12,32)";
        let (_, matched) = parse_operands(input).unwrap();
        assert_eq!(matched.0, 12);
        assert_eq!(matched.1, 32);
    }
    #[test]
    fn test_parse() {
        let test =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".chars();
        let operations = parse(&mut test.clone(), false).unwrap();
        let sum: i32 = operations.iter().sum();
        assert_eq!(sum, 161);
    }
    #[test]
    fn test_parse_cond() {
        let test =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".chars();
        let operations = parse(&mut test.clone(), true).unwrap();
        println!("{:?}", operations);
        let sum: i32 = operations.iter().sum();
        assert_eq!(sum, 48);
    }
}
