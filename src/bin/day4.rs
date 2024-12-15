use miette::Result;

type CharMatrix = Vec<Vec<char>>;
type CoordVec = Vec<(isize, isize)>;

const TEST: &str = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"#;

pub fn main() -> Result<()> {
    let data = std::fs::read_to_string("data/day4.txt").unwrap();
    let matrix = create_matrix(&data);
    let points = find_starting_points(&matrix.clone(), 'X');
    let result = find_matches(
        points,
        matrix.clone(),
        create_directions_vec(true),
        "XMAS".into(),
    );
    println!("Day 4, part 1: {result}");
    let points = find_starting_points(&matrix.clone(), 'A');
    let result = find_matches(
        points,
        matrix,
        create_directions_vec(false),
        "MASMAS".into(),
    );
    println!("Day 4, part 2: {result}");
    Ok(())
}

fn create_directions_vec(part_1: bool) -> Vec<Vec<(i32, i32)>> {
    if part_1 {
        return vec![
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
            vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
            vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            vec![(0, 0), (1, -1), (2, -2), (3, -3)],
        ];
    }
    return vec![
        vec![(1, -1), (0, 0), (-1, 1), (-1, -1), (0, 0), (1, 1)],
        vec![(1, 1), (0, 0), (-1, -1), (-1, 1), (0, 0), (1, -1)],
        vec![(-1, -1), (0, 0), (1, 1), (-1, 1), (0, 0), (1, -1)],
        vec![(1, 1), (0, 0), (-1, -1), (1, -1), (0, 0), (-1, 1)],
    ];
}

fn create_matrix(text: &str) -> CharMatrix {
    text.lines()
        .into_iter()
        .map(|line| {
            return line.trim().chars().collect::<Vec<char>>();
        })
        .collect()
}

fn find_starting_points(matrix: &CharMatrix, needle: char) -> Vec<CoordVec> {
    matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, ch)| {
                    if *ch == needle {
                        Some((row_idx as isize, col_idx as isize))
                    } else {
                        None
                    }
                })
                .collect::<CoordVec>()
        })
        .collect::<Vec<CoordVec>>()
}

fn check_direction(directions: &Vec<(isize, isize)>, matrix: &CharMatrix, needle: &str) -> bool {
    if directions
        .iter()
        .any(|direction| direction.0 < 0 || direction.1 < 0)
    {
        return false;
    }

    let query_matrix = |point: (usize, usize)| -> Option<&char> {
        return matrix.get(point.0).and_then(|row| row.get(point.1));
    };

    let checks: Vec<Option<&char>> = directions
        .iter()
        .map(|direction| {
            return query_matrix((direction.0 as usize, direction.1 as usize));
        })
        .collect();

    match checks.iter().all(|check| check.is_some()) {
        true => {
            let test = String::from_iter(checks.iter().map(|check| check.unwrap()));
            return test == needle;
        }
        _ => return false,
    }
}

fn find_matches(
    points: Vec<CoordVec>,
    matrix: CharMatrix,
    directions: Vec<Vec<(i32, i32)>>,
    needle: String,
) -> u16 {
    let mut result = 0;
    points.iter().flatten().for_each(|point| {
        result += directions
            .iter()
            .map(|direction| {
                let directions = direction
                    .iter()
                    .map(|d| (d.0 as isize + point.0, d.1 as isize + point.1))
                    .collect::<Vec<(isize, isize)>>();
                return check_direction(&directions, &matrix, &needle);
            })
            .filter(|x| *x)
            .map(|_| 1)
            .sum::<u16>();
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        let matrix = create_matrix(TEST);
        let points = find_starting_points(&matrix, 'X');
        let result = find_matches(points, matrix, create_directions_vec(true), "XMAS".into());
        assert_eq!(result, 18);
    }

    #[test]
    fn test_x_mas() {
        let matrix = create_matrix(TEST);
        let points = find_starting_points(&matrix, 'A');
        let result = find_matches(
            points.clone(),
            matrix.clone(),
            create_directions_vec(false),
            "MASMAS".into(),
        );
        assert_eq!(result, 9);
    }
}
