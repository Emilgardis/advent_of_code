
use y{{year}}_day{{day}}_{{title_snake}}::Solution;
use aoc::{Aoc, parts::*};


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    println!(":: ⭐Solution found⭐ ::\n{}", Aoc::solve::<Solution, Year{{year}}, Day{{day}}, Part1>()?);
    Ok(())
}

