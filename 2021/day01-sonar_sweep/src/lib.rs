//! Advent of code 2021 1
//!
//!! Link: <https://adventofcode.com/2021/day/1>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

pub type Depth = u32;

impl Solver<Year2021, Day1, Part1> for Solution {
    type Input<'a> = Vec<Depth>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input
            .lines()
            .map(|line| line.trim().parse::<Depth>())
            .collect::<Result<_, _>>()?)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        Ok(input.iter().tuple_windows().filter(|(a, b)| b > a).count())
    }
}

impl Solver<Year2021, Day1, Part2> for Solution {
    type Input<'a> = Vec<Depth>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2021, Day1, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let sums = input.iter().tuple_windows().map(|(a, b, c)| a + b + c);
        Ok(sums.tuple_windows().filter(|(a, b)| b > a).count())
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day1, Part1>(input)?,
        7
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day1, Part2>(input)?,
        5
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day1, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day1, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
