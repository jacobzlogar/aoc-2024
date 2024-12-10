use miette::IntoDiagnostic;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn next_direction(value: Direction) -> Direction {
    match value {
        Direction::West => Direction::North,
        Direction::East => Direction::South,
        Direction::North => Direction::East,
        Direction::South => Direction::West,
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Edge {
    point: Point,
    path: Vec<Point>,
}

impl From<&mut Lab> for Edge {
    fn from(lab: &mut Lab) -> Self {
        let current_edge = lab.guard.point;
        let mut points = vec![];
        let mut point = Point {
            x: current_edge.x,
            y: current_edge.y,
        };
        let path: Option<Vec<Point>> = match lab.guard.direction {
            Direction::East => {
                let mut obstructions = lab.obstructions.clone();
                obstructions.sort_by(|a, b| a.x.cmp(&b.x));
                obstructions
                    .iter()
                    .find_map(|obstruction| {
                        if obstruction.x > current_edge.x && obstruction.y == current_edge.y {
                            Some(obstruction)
                        } else {
                            None
                        }
                    })
                    .map(|result| {
                        let mut path = vec![];
                        point.x = result.x - 1;
                        for i in current_edge.x..point.x {
                            path.push(Point {
                                x: i,
                                y: current_edge.y,
                            })
                        }
                        return path;
                    })
            }
            Direction::West => {
                let mut obstructions = lab.obstructions.clone();
                obstructions.sort_by(|a, b| b.x.cmp(&a.x));
                obstructions
                    .iter()
                    .find_map(|obstruction| {
                        if obstruction.x < current_edge.x && obstruction.y == current_edge.y {
                            Some(obstruction)
                        } else {
                            None
                        }
                    })
                    .map(|result| {
                        let mut path = vec![];
                        point.x = result.x + 1;
                        for i in point.x..current_edge.x {
                            path.push(Point {
                                x: i,
                                y: current_edge.y,
                            })
                        }
                        return path;
                    })
            }
            Direction::North => {
                let mut obstructions = lab.obstructions.clone();
                obstructions.sort_by(|a, b| b.y.cmp(&a.y));
                obstructions
                    .iter()
                    .find_map(|obstruction| {
                        if obstruction.x == current_edge.x && obstruction.y < current_edge.y {
                            Some(obstruction)
                        } else {
                            None
                        }
                    })
                    .map(|result| {
                        let mut path = vec![];
                        point.y = result.y + 1;
                        for i in point.y..current_edge.y {
                            path.push(Point {
                                x: current_edge.x,
                                y: i,
                            })
                        }
                        return path;
                    })
            }
            Direction::South => {
                let mut obstructions = lab.obstructions.clone();
                obstructions.sort_by(|a, b| a.y.cmp(&b.y));
                obstructions
                    .iter()
                    .find_map(|obstruction| {
                        if obstruction.x == current_edge.x && obstruction.y > current_edge.y {
                            Some(obstruction)
                        } else {
                            None
                        }
                    })
                    .map(|result| {
                        let mut path = vec![];
                        point.y = result.y - 1;
                        for i in current_edge.y..point.y {
                            path.push(Point {
                                x: current_edge.x,
                                y: i,
                            })
                        }
                        return path;
                    })
            }
        };
        if let Some(paths) = path {
            paths.iter().for_each(|path| points.push(*path))
        }
        points.push(lab.guard.point);
        Self {
            path: points,
            point,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    point: Point,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Lab {
    guard: Guard,
    obstructions: Vec<Point>,
    visited: Vec<Point>,
    height: usize,
    width: usize,
}

impl Lab {
    fn parse(text: &str) -> Self {
        let mut obstructions = vec![];
        let mut guard = None;
        for (row_index, row) in text.lines().enumerate() {
            for (col_index, column) in row.chars().enumerate() {
                match column {
                    '#' => obstructions.push(Point {
                        x: col_index,
                        y: row_index,
                    }),
                    '^' => {
                        guard = Some(Guard {
                            point: Point {
                                x: col_index,
                                y: row_index,
                            },
                            direction: Direction::North,
                        })
                    }
                    _ => (),
                }
            }
        }
        let guard = guard.expect("Couldn't find guard?");

        Self {
            obstructions,
            guard,
            visited: vec![],
            height: text.lines().count(),
            width: text.lines().nth(1).unwrap().len(),
        }
    }

    fn unobstructed(&mut self, point: Point) -> bool {
        point.x == 0 || point.y == 0 || point.x == self.width || point.y == self.height
    }

    fn add_points(&mut self, points: Vec<Point>) {
        for point in points {
            if !self.visited.contains(&point) {
                self.visited.push(point);
            }
        }
    }

    fn print_map(&mut self, part_1: bool) {
        print!("  ");
        for i in 0..self.width {
            print!("{i}");
        }
        println!("");
        for i in 0..self.height {
            print!("{i}|");
            for j in 0..self.width {
                let point = Point { x: j, y: i };
                let visited = self.visited.contains(&point);
                let obstructed = self.obstructions.contains(&point);
                let guarded = self.guard.point == point;
                match (visited, obstructed, guarded) {
                    (false, true, false) => {
                        print!("#")
                    }
                    (true, false, true) => match self.guard.direction {
                        Direction::West => print!("^"),
                        Direction::East => print!(">"),
                        Direction::North => print!("^"),
                        Direction::South => print!("v"),
                    },
                    (true, false, false) => {
                        print!("X");
                    }
                    _ => print!("."),
                }
            }
            println!("");
        }
    }

    fn exit(&mut self) -> Vec<Point> {
        let mut exit_path = vec![];
        match self.guard.direction {
            Direction::East => {
                for i in self.guard.point.x..self.width {
                    exit_path.push(Point {
                        x: i,
                        y: self.guard.point.y,
                    })
                }
            }
            Direction::West => {
                for i in 0..self.guard.point.x {
                    exit_path.push(Point {
                        x: i,
                        y: self.guard.point.y,
                    })
                }
            }
            Direction::South => {
                for i in self.guard.point.y..self.height {
                    exit_path.push(Point {
                        x: self.guard.point.x,
                        y: i,
                    })
                }
            }
            Direction::North => {
                for i in 0..self.height {
                    exit_path.push(Point {
                        x: self.guard.point.x,
                        y: i,
                    })
                }
            }
        }
        exit_path
    }

    // At each obstruction we need to:
    // - re-orient the guard's direction
    // - find the points between guard and next obstruction
    // - add those points to the vec of visited points (if they don't exist already)
    // if there is no obstruction in the guards direction we need to:
    // - find the points between the guard and the edge of the map
    // - add those points to the vec of visited points
    #[tracing::instrument(skip_all)]
    fn traverse(&mut self) {
        while !self.unobstructed(self.guard.point) {
            let edge = Edge::from(&mut self.clone());
            if edge.path.len() == 1 {
                break;
            }
            self.add_points(edge.path.clone());
            self.guard.point = edge.point;
            self.guard.direction = next_direction(self.guard.direction);
        }
        let exit_path = self.exit();
        self.guard.point = *exit_path.last().unwrap();
        self.add_points(exit_path);
    }
}

pub fn run() -> miette::Result<()> {
    fmt::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_level(false)
        .init();
    let data = std::fs::read_to_string("data/day6.txt").into_diagnostic()?;
    let mut lab = Lab::parse(&data);
    println!("Day 6, part 1: {}", part_1(&mut lab));
    Ok(())
}

#[tracing::instrument(skip_all)]
fn part_1(lab: &mut Lab) -> usize {
    lab.traverse();
    lab.visited.len()
}

#[tracing::instrument(skip_all)]
fn part_2(lab: &mut Lab) -> usize {
    1
}

const TEST: &str = r#"....#.....
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
    let mut lab = Lab::parse(TEST);
    lab.traverse();
    lab.print_map(true);
    assert_eq!(lab.visited.len(), 41);
}

#[cfg(test)]
#[test]
fn test_day6_part2() {
    let mut lab = Lab::parse(TEST);
    lab.print_map(false);
    lab.traverse();
}
