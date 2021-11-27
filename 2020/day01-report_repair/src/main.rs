use y2020_day1_report_repair::Solution;
use aoc::{Aoc, parts::*};

pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    println!("⭐Solution found⭐ ::\n{}", Aoc::solve::<Solution, Year2020, Day1, Part1>()?);
    Ok(())
}
