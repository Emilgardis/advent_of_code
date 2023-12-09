//! Advent of code 2022 4
//!
//!! Link: <https://adventofcode.com/2022/day/4>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2022, Day4, Part1> for Solution {
    type Input<'a> = Vec<((u32, u32), (u32, u32))>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        fn split_str(s: &str) -> Result<(u32, u32), Report> {
            s.split_once('-')
                .ok_or_else(|| eyre::eyre!("Invalid input: {s}"))
                .map(|(a, b)| Ok((a.parse()?, b.parse()?)))?
        }
        input
            .lines()
            .map(|s| {
                s.trim()
                    .split_once(',')
                    .ok_or_else(|| eyre::eyre!("Invalid input: {s}"))
            })
            .map_ok(|(a, b)| Ok((split_str(a)?, split_str(b)?)))
            .flatten()
            .collect::<Result<Vec<((u32, u32), (u32, u32))>, _>>()
    }

    fn solve(input: &Vec<((u32, u32), (u32, u32))>) -> Result<Self::Output, Report> {
        let mut count = 0;

        for (a, b) in input {
            let a = a.0..=a.1;
            let b = b.0..=b.1;
            if a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
            {
                count += 1;
            }
        }
        Ok(count)
    }
}

impl Solver<Year2022, Day4, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 4, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 4, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day4, Part1>>::generate_input(input)
    }

    fn solve(input: &Vec<((u32, u32), (u32, u32))>) -> Result<Self::Output, Report> {
        let mut count = 0;

        for (a, b) in input {
            let a = a.0..=a.1;
            let b = b.0..=b.1;
            if a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
            {
                count += 1;
            }
        }
        Ok(count)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day4, Part1>(input)?,
        2
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day4, Part2>(input)?,
        4
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day4, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day4, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
