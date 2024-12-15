pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let mut first_group: Vec<i32> = vec![];
    let mut second_group: Vec<i32> = vec![];
    let mut result: i32 = 0;
    let challenge = std::fs::read_to_string("data/day1.txt")?;
    for (index, text) in challenge.split_whitespace().enumerate() {
        if index % 2 == 0 {
            first_group.push(text.parse::<i32>().unwrap());
        } else {
            second_group.push(text.parse::<i32>().unwrap());
        }
    }
    first_group.sort();
    second_group.sort();
    for (index, value) in first_group.iter().enumerate() {
        let r = (value - second_group[index]).abs();
        result += r;
    }
    println!("Day 1, part 1: {:?}", result);
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut first_group: Vec<i32> = vec![];
    let mut second_group: Vec<i32> = vec![];
    let mut result: i32 = 0;
    let challenge = std::fs::read_to_string("data/day1.txt")?;
    for (index, text) in challenge.split_whitespace().enumerate() {
        if index % 2 == 0 {
            first_group.push(text.parse::<i32>().unwrap());
        } else {
            second_group.push(text.parse::<i32>().unwrap());
        }
    }
    first_group.sort();
    second_group.sort();
    for value in first_group.iter() {
        let cnt = second_group.iter().filter(|s| *s == value).count();
        result += value * cnt as i32;
    }
    println!("Day 1, part 2: {:?}", result);
    Ok(())
}
