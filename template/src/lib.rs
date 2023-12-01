//! Advent of code {{year}} {{day}}
//!
//! Link: <https://adventofcode.com/{{year}}/day/{{day}}>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//! {{brief}}
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *


use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year{{year}}, Day{{day}}, Part1> for Solution {
    type Input<'a> = Vec<&'a str>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.lines().map(|s| s.trim()).collect())
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        todo!()
    }
}

impl Solver<Year{{year}}, Day{{day}}, Part2> for Solution {
    type Input<'a> = <Self as Solver<{{year}}, {{day}}, Part1>>::Input<'a>;

    type Output = <Self as Solver<{{year}}, {{day}}, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year{{year}}, Day{{day}}, Part1>>::generate_input(input)
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        todo!()
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
0
    "#.trim();
    assert_eq!(aoc::solve_with_input::<Solution, Year{{year}}, Day{{day}}, Part1>(input)?, 0);
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
0
    "#.trim();
    assert_eq!(aoc::solve_with_input::<Solution, Year{{year}}, Day{{day}}, Part2>(input)?, 0);
    Ok(())
}


#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year{{year}}, Day{{day}}, Part1>().map(|s| {println!(":: ⭐Solution found⭐ ::\n{s}")})
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report>{
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year{{year}}, Day{{day}}, Part2>().map(|s| {println!(":: ⭐Solution found⭐ ::\n{s}")})
}
