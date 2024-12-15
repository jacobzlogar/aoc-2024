use miette::IntoDiagnostic;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn get_input(day: &str) -> miette::Result<String> {
    let base = env!("CARGO_MANIFEST_DIR");
    let data = std::fs::read_to_string(format!("{}/data/{}.txt", base, day)).into_diagnostic()?;
    Ok(data)
}
