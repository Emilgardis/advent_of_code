
use y2020_day3_toboggan_trajectory::Solution;
use aoc::{Aoc, parts::*};


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    println!(":: ⭐Solution found⭐ ::\n{}", Aoc::solve::<Solution, Year2020, Day3, Part2>()?);
    Ok(())
}

