use std::collections::HashMap;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Point = (u32, u32);
pub struct TrailHead {
    map: Vec<Vec<u32>>,
    result: usize,
}

impl TrailHead {
    pub fn parse(text: &str) -> Self {
        let mut map = vec![];
        for (i, line) in text.lines().enumerate() {
            let mut values = vec![];
            for (j, ch) in line.chars().enumerate() {
                let digit = ch.to_digit(10).unwrap();
                values.push(digit);
            }
            map.push(values)
        }
        Self { map, result: 0 }
    }

    pub fn dfs(
        &mut self,
        point: Point,
        map: Vec<Vec<u32>>,
        count: u32,
    ) -> usize {
        let active = map[point.0 as usize][point.1 as usize];
        if active == 9 {
            return 1;
        }
        1
        // let has_adjacent = |x: &(i32, i32), index| {
        //     let others = map.get(&index).unwrap();
        //     return others
        //         .iter()
        //         .filter(|other| {
        //             DIRECTIONS.iter().any(move |direction| {
        //                 let check = (other.0 + direction.0, other.1 + direction.1);
        //                 *x == check
        //             })
        //         })
        //         .collect::<Vec<&(i32, i32)>>();
        // };
        // let paths = has_adjacent(&point, index);
        // if paths.len() > 0 {
        //     for path in paths {
        //         visited.push(*path);
        //         return self.dfs(self.map.clone(), visited, *path, index + 1);
        //     }
        // }
        // None
    }

    #[tracing::instrument(skip_all)]
    pub fn solve(&mut self) -> usize {
        let mut start = self.map.get(&0).unwrap().clone();
        for point in start {
            self.result += self.dfs(self.map.clone(), &mut vec![point], point, 1);
        }
        self.result
    }
}

pub fn run() -> miette::Result<()> {
    let data = crate::get_input("day10")?;
    println!("Day 10, part 1: {}", part_1(&data));
    Ok(())
}

fn part_1(data: &str) -> usize {
    let mut trailhead = TrailHead::parse(data);
    trailhead.solve()
}

const TEST: &str =
r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

#[cfg(test)]
#[test]
fn test_day10_part1() {
    println!("{:?}", part_1(TEST));
    //assert_eq!(part_1(TEST), 1928);
}
