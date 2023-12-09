#![feature(generic_associated_types)]
//! Advent of code 2021 5
//!
//! https://adventofcode.com/2021/day/5
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2021, Day5, Part1> for Solution {
    type Input<'a> = ();

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        todo!()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        todo!()
    }
}

impl Solver<Year2021, Day5, Part2> for Solution {
    type Input<'a> = ();

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2021, Day5, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        todo!()
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day5, Part1>(input)?,
        5
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day5, Part2>(input)?,
        0
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day5, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day5, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}
