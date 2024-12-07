use miette::IntoDiagnostic;

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Guard {
    point: Point,
    direction: Direction
}

#[derive(Debug)]
struct Lab {
    obstructions: Vec<Point>,
    visited: Vec<Point>,
    length: usize,
    width: usize,
}

impl Lab {
    fn parse(text: &str) -> (Self, Guard) {
        let mut obstructions = vec![];
        let mut guard = None;
        for (row_index, row) in text.lines().enumerate() {
            for (col_index, column) in row.chars().enumerate() {
                match column {
                    '#' => obstructions.push(Point { x: col_index, y: row_index }),
                    '^' => guard = Some(Guard {
                        point: Point { x: col_index, y: row_index },
                        direction: Direction::North
                    }),
                    _ => ()
                }
            }
        }
        let lab = Self {
            obstructions,
            visited: vec![],
            length: text.lines().count() - 1,
            width: text.lines().nth(1).unwrap().len() - 1,
        };
        let guard = guard.expect("Couldn't find guard?");
        (lab, guard)
    }

    fn is_obstructed(&mut self, guard: &Guard) -> bool {
        guard.point.x != self.width
            || guard.point.y != self.length
            || guard.point.x != 0
            || guard.point.y != 0
    }

    fn traverse(&mut self, mut guard: Guard) {
        while self.is_obstructed(&guard) {
            match guard.direction {
                Direction::West => {
                    let obstruction = self.obstructions.iter().find(|obstruction| {
                        return obstruction.y == guard.point.y && obstruction.x < guard.point.x;   
                    });
                    if let Some(next) = obstruction {
                        for i in next.x..guard.point.x {
                            let point = Point {
                                x: i,
                                y: guard.point.y
                            };
                            if !self.visited.contains(&point) {
                                self.visited.push(point);
                            }
                        }
                        guard.point = Point {
                            x: next.x + 1,
                            y: next.y
                        };
                    } else {
                        for i in guard.point.x..self.width {
                            let point = Point {
                                x: i,
                                y: guard.point.y
                            };
                            self.visited.push(point);
                        }
                        break;
                    }
                    guard.direction = Direction::North;
                },
                Direction::North => {
                    let obstruction = self.obstructions.iter().find(|obstruction| {
                        return obstruction.x == guard.point.x && obstruction.y < guard.point.y;
                    });
                    if let Some(next) = obstruction {
                        for i in guard.point.y..next.y {
                            let point = Point {
                                y: i,
                                x: guard.point.x
                            };
                            self.visited.push(point);
                        }
                        guard.point = Point {
                            x: next.x,
                            y: next.y + 1
                        };
                    } else {
                        for i in guard.point.y..self.length {
                            let point = Point {
                                y: i,
                                x: guard.point.x
                            };
                            self.visited.push(point);
                        }
                        break;
                    }
                    guard.direction = Direction::East;
                },
                Direction::East => {
                    let obstruction = self.obstructions.iter().find(|obstruction| {
                        return  obstruction.y == guard.point.y && obstruction.x > guard.point.x;   
                    });
                    if let Some(next) = obstruction {
                        for i in guard.point.x..next.x {
                            println!("{i}");
                            let point = Point {
                                x: i,
                                y: guard.point.y
                            };
                            if !self.visited.contains(&point) {
                                self.visited.push(point);
                            }
                        }
                        guard.point = Point {
                            x: next.x - 1,
                            y: next.y
                        };
                    } else {
                        for i in guard.point.x..self.width {
                            let point = Point {
                                x: i,
                                y: guard.point.y
                            };
                            self.visited.push(point);
                        }
                        break;
                    }
                    guard.direction = Direction::South;
                },
                Direction::South => {
                    let obstruction = self.obstructions.iter().find(|obstruction| {
                        return obstruction.x == guard.point.x && obstruction.y > guard.point.y;
                    });
                    if let Some(next) = obstruction {
                        for i in guard.point.y..next.y {
                            println!("{i}");
                            let point = Point {
                                y: i,
                                x: guard.point.x
                            };
                            if !self.visited.contains(&point) {
                                self.visited.push(point);
                            }
                        }
                        guard.point = Point {
                            x: next.x,
                            y: next.y - 1
                        };
                    } else {
                        for i in guard.point.y..self.length {
                            println!("{i}, {:?}", self.visited.len());
                            let point = Point {
                                y: i,
                                x: guard.point.x
                            };
                            self.visited.push(point);
                        }
                        break;
                    }
                    guard.direction = Direction::West;
                },
            }
        }
        println!("{:?}", guard);
    }
}

pub fn run() -> miette::Result<()> {
    let data = std::fs::read_to_string("data/day6.txt")
        .into_diagnostic()?;
    let (mut lab, guard) = Lab::parse(&data);
    lab.traverse(guard);
    println!("{:?}", lab.visited.len());
    Ok(())
}

fn part_1() {
}

fn part_2() {
}

const TEST: &str =
    r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[cfg(test)]
#[test]
fn test_day6_part1() {
    let (mut lab, guard) = Lab::parse(TEST);
    lab.traverse(guard);
    println!("{:?}", lab.visited.len());
}
