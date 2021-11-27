
use y2020_day2_password_philosophy::Solution;
use aoc::{Aoc, parts::*};


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    let aoc = aoc::Aoc::new(&2020, &2)?;
    println!(":: ⭐Solution found⭐ ::\n{}", Aoc::solve::<Solution, Year2020, Day2, Part1>()?);
    Ok(())
}

