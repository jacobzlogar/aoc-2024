use miette::{miette, IntoDiagnostic, Result};
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string("data/day5.txt").into_diagnostic()?;
    let mut queue = PrintQueue::parse(&input);
    println!("Day 5, part 1: {:?}", solve(&mut queue, false));
    queue.total = 0;
    println!("Day 5, part 2: {:?}", solve(&mut queue, true));
    Ok(())
}

#[derive(Debug)]
struct PrintQueue {
    pages: Vec<Vec<u32>>,
    updates: Vec<(u32, u32)>,
    total: u32,
}

impl PrintQueue {
    fn parse(text: &str) -> PrintQueue {
        let lines = text.lines().into_iter();
        let pages = lines
            .clone()
            .filter_map(|line| {
                if let Ok(page) = parse_page(line) {
                    return Some(page);
                }
                None
            })
            .collect::<Vec<Vec<u32>>>();
        let updates = lines
            .clone()
            .filter_map(|line| {
                if let Ok(update) = parse_update(line) {
                    return Some(update);
                }
                None
            })
            .collect::<Vec<(u32, u32)>>();
        let queue = PrintQueue {
            pages,
            updates,
            total: 0,
        };
        queue
    }
}

fn parse_update(text: &str) -> Result<(u32, u32)> {
    let res: Result<Vec<u32>> = text
        .split("|")
        .into_iter()
        .map(|n| n.parse::<u32>().into_diagnostic())
        .collect();
    match res {
        Err(e) => {
            return Err(miette!(e));
        }
        Ok(o) => return Ok((o[0], o[1])),
    }
}

fn parse_page(text: &str) -> Result<Vec<u32>> {
    text.split(",")
        .into_iter()
        .map(|n| n.parse::<u32>().into_diagnostic())
        .collect::<Result<Vec<u32>>>()
}

fn solve(queue: &mut PrintQueue, part_2: bool) -> u32 {
    for page in &queue.pages {
        let matched = page.iter().enumerate().all(|(index, point)| {
            let (left, right) = page.split_at(index);
            let right = &right[1..];
            let before = left.iter().all(|left| {
                queue
                    .updates
                    .iter()
                    .find(|(page_left, page_right)| *page_right == *point && *page_left == *left)
                    .is_some()
            });
            let after = right.iter().all(|right| {
                queue
                    .updates
                    .iter()
                    .find(|(page_left, page_right)| *page_right == *right && *page_left == *point)
                    .is_some()
            });
            return before && after;
        });
        if matched && !part_2 {
            let (_, right) = page.split_at(page.len() / 2);
            queue.total += right[0] as u32;
        } else if !matched && part_2 {
            // find the updates where both values are part of the current page
            // 75, 97, 47, 61, 53
            // would return the following tuples
            // (47, 53), (97, 61), (97, 47), (75, 53), (61, 53), (97, 53), (97, 75), (47, 61), (75, 61)
            // from here we need to determine the correct possible ordering, regardless of initial ordering
            let updates: Vec<(u32, u32)> = queue
                .updates
                .clone()
                .into_iter()
                .filter(|(a, b)| page.contains(a) && page.contains(b))
                .collect();

            let mut map: HashMap<u32, u32> = HashMap::new();
            updates.iter().for_each(|(_, y)| {
                *map.entry(*y).or_insert(0) += 1;
            });
            let mut new_page: Vec<u32> = page.clone();
            new_page.sort_by(|a, b| map.get(a).unwrap_or(&0).cmp(&map.get(b).unwrap_or(&0)));
            let (_, right) = new_page.split_at(page.len() / 2);
            queue.total += right[0] as u32;
        }
    }
    queue.total
}

const TEST: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

#[cfg(test)]
#[test]
fn test_day5_part1() {
    let mut queue = PrintQueue::parse(TEST);
    assert_eq!(solve(&mut queue, false), 143);
}

#[test]
fn test_day5_part2() {
    let mut queue = PrintQueue::parse(TEST);
    assert_eq!(solve(&mut queue, true), 123);
}
