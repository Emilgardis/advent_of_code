#![feature(iter_array_chunks)]
//! Advent of code 2022 6
//!
//!! Link: <https://adventofcode.com/2022/day/6>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2022, Day6, Part1> for Solution {
    type Input<'a> = &'a str;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.trim())
    }

    fn solve(input: &&'_ str) -> Result<Self::Output, Report> {
        input
            .chars()
            .tuple_windows()
            .position(|(a, b, c, d)| [a, b, c, d].iter().all_unique())
            .ok_or_else(|| eyre::eyre!("none found"))
            .map(|p| p + 4)
    }
}

impl Solver<Year2022, Day6, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 6, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 6, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day6, Part1>>::generate_input(input)
    }

    fn solve(input: &&str) -> Result<Self::Output, Report> {
        let vec: Vec<_> = input.chars().collect();
        vec.windows(14)
            .position(|a| a.iter().all_unique())
            .ok_or_else(|| eyre::eyre!("none found"))
            .map(|p| p + 14)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>("bvwbjplbgvbhsrlpgdmjqwftvncz")?,
        5
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>("nppdvjthqldpwncqszvftbrmjlhg")?,
        6
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        )?,
        10
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        )?,
        11
    );

    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();

    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?,
        19
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("bvwbjplbgvbhsrlpgdmjqwftvncz")?,
        23
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("nppdvjthqldpwncqszvftbrmjlhg")?,
        23
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        )?,
        29
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        )?,
        26
    );

    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day6, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day6, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
