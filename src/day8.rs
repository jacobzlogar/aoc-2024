pub fn run() -> miette::Result<()> {
    let data = crate::get_input("day8")?;
    println!("Day 8, part 1: {:?}", part_1(&data));
    println!("Day 8, part 2: {:?}", part_2(&data));
    Ok(())
}

fn part_1(data: &str) -> usize {
    let mut roof = Roof::parse(&data);
    roof.solve(false);
    roof.antinode_map.len()
}

fn part_2(data: &str) -> usize {
    let mut roof = Roof::parse(&data);
    roof.solve(true);
    roof.print_map();
    roof.antinode_map.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: isize,
    y: isize,
    symbol: char
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Roof {
    antenna_map: Vec<Point>,
    antinode_map: Vec<Point>,
    width: isize,
    height: isize,
}

impl Roof {
    fn antinode_exists(&mut self, point: &Point) -> Option<Point> {
        self.antinode_map
            .clone()
            .into_iter()
            .find(|pt| pt.x == point.x && pt.y == point.y)
    }

    fn print_map(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let antenna_at_point = self.antenna_map
                    .iter()
                    .find(|antenna| antenna.x == x && antenna.y == y);
                let antinode_at_point = self.antinode_map
                    .iter()
                    .find(|antenna| antenna.x == x && antenna.y == y);
                match (antenna_at_point, antinode_at_point) {
                    (Some(antenna), _) => print!("{}", antenna.symbol),
                    (None, Some(_)) => print!("#"),
                    _ => print!(".")
                }
            }
            println!("");
        }
    }

    #[tracing::instrument(skip_all)]
    fn solve(&mut self, repeat: bool) {
        let height = self.height.clone();
        let width = self.height.clone();
        for antenna in self.antenna_map.clone().into_iter() {
            let antennas = self.antenna_map.clone();
            let other_antennas = antennas.iter().filter(|&other| {
                return other.x != antenna.x && other.y != antenna.y && other.symbol == antenna.symbol;
            });
            for other_antenna in other_antennas {
                let diff = (antenna.x - other_antenna.x, antenna.y - other_antenna.y);
                let x = antenna.x + diff.0;
                let y = antenna.y + diff.1;

                let within_bounds = |x, y| x >= 0 && x < width && y >= 0 && y < height;

                if repeat {
                    for i in 0..height {
                        let x = antenna.x + diff.0 * i;
                        let y = antenna.y + diff.1 * i;
                        if within_bounds(x, y) {
                            let point = Point { x, y, symbol: '#' };
                            if self.antinode_exists(&point).is_none() {
                                self.antinode_map.push(point);
                            }
                        }
                        println!("{x} {y}");
                    }
                }
                if within_bounds(x, y) {
                    let point = Point { x, y, symbol: '#' };
                    if self.antinode_exists(&point).is_none() {
                        self.antinode_map.push(point);
                    }
                }
            }
        }
    }

    
    fn parse(text: &str) -> Self {
        let mut antenna_map = vec![];
        let height = text.lines().count() as isize;
        let width = text.lines().nth(0).unwrap().len() as isize; 
        for (row, line) in text.lines().enumerate() {
            for (column, ch) in line.chars().enumerate() {
                if ch == '.' {
                    continue;
                }
                let point = Point { x: column as isize, y: row as isize, symbol: ch };
                antenna_map.push(point);
            }
        }
        Self {
            antenna_map,
            antinode_map: vec![],
            height,
            width,
        }
    }
}

const TEST: &str =
r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
#[cfg(test)]
#[test]
fn test_day8_part1() {
    assert_eq!(part_1(TEST), 14);
}

const TEST_PART2 : &str =
r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
"#;
#[test]
fn test_day8_part2() {
    assert_eq!(part_2(TEST), 9);
}
