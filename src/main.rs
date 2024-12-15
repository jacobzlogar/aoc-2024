use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

use aoc_2024::day1;
use aoc_2024::day10;
use aoc_2024::day11;
use aoc_2024::day12;
use aoc_2024::day13;
use aoc_2024::day2;
use aoc_2024::day3;
use aoc_2024::day4;
use aoc_2024::day5;
use aoc_2024::day6;
use aoc_2024::day7;
use aoc_2024::day8;
use aoc_2024::day9;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_level(false)
        .init();
    // day1::part1()?;
    // day1::part2()?;
    // day2::run()?;
    // day3::run()?;
    // day4::run()?;
    // day5::run()?;
    // day6::run()?;
    // day7::run()?;
    // day8::run()?;
    // day9::run()?;
    // day10::run()?;
    day11::run()?;
    Ok(())
}
