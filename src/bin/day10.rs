use std::collections::HashSet;

const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Point = (usize, usize);
pub struct TrailHead {
    map: Vec<Vec<u32>>,
    width: usize,
    length: usize,
    result: u32,
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
        let width = map.len();
        let length = map[0].len();
        Self {
            map,
            width,
            length,
            result: 0,
        }
    }

    pub fn dfs(
        &mut self,
        point: Point,
        map: &Vec<Vec<u32>>,
        visited: &mut HashSet<Point>,
        rating: bool,
        mut count: u32,
    ) -> u32 {
        if !rating {
            if visited.contains(&point) {
                return 0;
            }
            visited.insert(point);
        }

        let active = map[point.0][point.1];
        if active == 9 {
            return 1;
        }

        let points: Vec<(usize, usize)> = DIRECTIONS
            .iter()
            .filter_map(|direction| {
                if let Ok(dx) = usize::try_from(direction.0 + point.0 as i8) {
                    if let Ok(dy) = usize::try_from(direction.1 + point.1 as i8) {
                        return Some((dx, dy));
                    }
                }
                None
            })
            .filter(|&(dx, dy)| {
                return dx <= self.width - 1 && dy <= self.length - 1 && map[dx][dy] == active + 1;
            })
            .collect();

        for point in points {
            count += self.dfs(point, map, visited, rating, 0);
        }
        count
    }

    #[tracing::instrument(skip_all)]
    pub fn solve(&mut self, rating: bool) -> u32 {
        for (row_index, row) in self.map.clone().iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if col == &0 {
                    let mut visited: HashSet<Point> = HashSet::new();
                    self.result += self.dfs(
                        (row_index, col_index),
                        &self.map.clone(),
                        &mut visited,
                        rating,
                        0,
                    );
                }
            }
        }
        self.result
    }
}

pub fn main() -> miette::Result<()> {
    let data = aoc_2024::get_input("day10")?;
    println!("Day 10, part 1: {}", part_1(&data));
    println!("Day 10, part 2: {}", part_2(&data));
    Ok(())
}

fn part_1(data: &str) -> u32 {
    let mut trailhead = TrailHead::parse(data);
    trailhead.solve(false)
}

fn part_2(data: &str) -> u32 {
    let mut trailhead = TrailHead::parse(data);
    trailhead.solve(true)
}

const TEST: &str = r#"89010123
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
    assert_eq!(part_1(TEST), 36);
}
#[test]
fn test_day10_part2() {
    assert_eq!(part_2(TEST), 81);
}
