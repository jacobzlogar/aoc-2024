use miette::IntoDiagnostic;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Calibration {
    values: Vec<usize>,
}

impl Calibration {
    fn sum(&self) -> usize {
        self.values.iter().sum()
    }

    fn product(&self) -> usize {
        self.values.iter().fold(1, |acc, e| acc * e)
    }
}

#[derive(Debug, Clone)]
struct Equation {
    left: usize,
    right: Calibration,
}

#[derive(Debug)]
struct Bridge {
    equations: Vec<Equation>,
    result: usize,
}

impl Bridge {
    fn parse(text: &str) -> Self {
        let equations = text
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(":").unwrap();
                let left = left.parse::<usize>().unwrap();
                let values: Vec<usize> = right
                    .trim()
                    .split(" ")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();
                let right = Calibration { values };
                return Equation { left, right };
            })
            .collect();
        Self {
            equations,
            result: 0,
        }
    }
}

pub fn run() -> miette::Result<()> {
    let data = &std::fs::read_to_string("data/day7.txt").into_diagnostic()?;
    let mut bridge = Bridge::parse(&data);
    println!("Day 7, part 1: {}", solve(&mut bridge, false));
    bridge.result = 0;
    println!("Day 7, part 2: {}", solve(&mut bridge, true));
    Ok(())
}

fn find_recursive(
    equation: &mut Equation,
    concat: bool,
    index: usize,
    current: usize,
) -> Option<usize> {
    if index == equation.right.values.len() {
        if current == equation.left {
            return Some(current);
        }
        return None;
    }

    let number = equation.right.values[index];

    if concat {
        let result_concat = format!("{current}{number}").parse::<usize>().unwrap();
        if let Some(result) = find_recursive(equation, concat, index + 1, result_concat) {
            return Some(result);
        }
    }

    let result_add = current + number;
    if let Some(result) = find_recursive(equation, concat, index + 1, result_add) {
        return Some(result);
    }

    let result_multiply = current * number;
    if let Some(result) = find_recursive(equation, concat, index + 1, result_multiply) {
        return Some(result);
    }

    None
}

#[tracing::instrument(skip_all)]
fn solve(bridge: &mut Bridge, concat: bool) -> usize {
    for equation in &bridge.equations {
        if equation.right.sum() == equation.left || equation.right.product() == equation.left {
            bridge.result += equation.left;
        } else if find_recursive(&mut equation.clone(), concat, 0, 0).is_some() {
            bridge.result += equation.left;
        }
    }
    bridge.result
}

const TEST: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

#[cfg(test)]
#[test]
fn test_day7_part1() {
    let mut bridge = Bridge::parse(TEST);
    assert_eq!(solve(&mut bridge, false), 3749);
}
#[test]
fn test_day7_part2() {
    let mut bridge = Bridge::parse(TEST);
    assert_eq!(solve(&mut bridge, true), 11387);
}
