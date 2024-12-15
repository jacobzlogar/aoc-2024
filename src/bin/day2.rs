fn differs_by_3(vec: &[i32]) -> bool {
    let direction = vec[1] - vec[0];
    vec.windows(2).all(|pair| {
        let mut cmp = false;
        if let [a, b] = pair {
            if direction > 0 {
                cmp = b > a && (b - a) <= 3;
            } else {
                cmp = a > b && (a - b) <= 3;
            }
        }
        cmp
    })
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part_1 = 0;
    let mut part_2 = 0;
    let f = std::fs::read_to_string("data/day2-rick.txt")?;
    for line in f.lines() {
        let levels = line
            .split_whitespace()
            .into_iter()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if differs_by_3(&levels) {
            part_1 += 1;
        }

        if differs_by_3(&levels) {
            part_2 += 1;
        } else {
            let len = levels.clone().len();
            for skip in 0..len {
                let dampened = levels
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != skip)
                    .map(|(_, val)| val)
                    .collect::<Vec<i32>>();
                if differs_by_3(&dampened) {
                    part_2 += 1;
                    break;
                }
            }
        }
    }
    println!("Day 2, part 1: {part_1}");
    println!("Day 2, part 2: {part_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::differs_by_3;

    #[test]
    fn test_reports() {
        let data = [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ];
        let sum = data.iter().fold(0, |mut acc, x| {
            if differs_by_3(x) {
                acc += 1;
            }
            acc
        });
        assert_eq!(sum, 2);
    }
}
