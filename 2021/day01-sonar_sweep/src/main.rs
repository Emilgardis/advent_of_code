
use y2021_day1_sonar_sweep::Solution;
use aoc::{Aoc, parts::*};


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    println!(":: ⭐Solution found⭐ ::\n{}", Aoc::solve::<Solution, Year2021, Day1, Part1>()?);
    Ok(())
}

