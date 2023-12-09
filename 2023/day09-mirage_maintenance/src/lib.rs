#![feature(iter_map_windows)]
//! Advent of code 2023 9
//!
//! Link: <https://adventofcode.com/2023/day/9>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2023, Day9, Part1> for Solution {
    type Input<'a> = Vec<Vec<i64>>;

    type Output = i64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input
            .lines()
            .map(|s| {
                s.trim()
                    .split_ascii_whitespace()
                    .map(|i| i.parse())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for oasis_line in input {
            let mut lines = vec![oasis_line.clone()];
            loop {
                lines.push(
                    lines
                        .last()
                        .unwrap()
                        .iter()
                        .map_windows(|[a, b]| *b - *a)
                        .collect_vec(),
                );
                if lines.last().unwrap().iter().all(|a| a == &0) {
                    println!("{:?}", lines);
                    break;
                }
            }
            let mut solutions = vec![0];
            for next in lines.iter().rev().skip(1) {
                solutions.push(next.last().unwrap() + solutions.last().unwrap());
                println!("next: {next:?}, sols: {solutions:?}");
            }
            sum += *solutions.last().unwrap();
        }
        Ok(sum)
    }
}

impl Solver<Year2023, Day9, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 9, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 9, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day9, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for oasis_line in input {
            let mut lines = vec![oasis_line.clone()];
            loop {
                lines.push(
                    lines
                        .last()
                        .unwrap()
                        .iter()
                        .map_windows(|[a, b]| *b - *a)
                        .collect_vec(),
                );
                if lines.last().unwrap().iter().all(|a| a == &0) {
                    println!("{:?}", lines);
                    break;
                }
            }
            let mut solutions = vec![0];
            for next in lines.iter().rev().skip(1) {
                solutions.push(next.first().unwrap() - solutions.last().unwrap());
                println!("next: {next:?}, sols: {solutions:?}");
            }
            sum += *solutions.last().unwrap();
        }
        Ok(sum)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day9, Part1>(input)?,
        114
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day9, Part2>(input)?,
        2
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day9, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day9, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
