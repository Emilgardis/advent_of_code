//! Advent of code 2020 1
//!
//!! Link: <https://adventofcode.com/2020/day/1>
//!
//! Good luck!

use aoc::{parts::*, Solver};
use eyre::Report;

impl Solver<Year2020, Day1, Part1> for Solution {
    type Input<'a> = Vec<usize>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input
            .split_whitespace()
            .map(|i| i.parse())
            .collect::<Result<_, _>>()?)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        for (a_index, a) in input.iter().enumerate() {
            for b in input.iter().skip(a_index + 1) {
                if a + b == 2020 {
                    return Ok(a * b);
                }
            }
        }
        Err(eyre::eyre!("no solution found"))
    }
}

impl Solver<Year2020, Day1, Part2> for Solution {
    type Input<'a> = Vec<usize>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2020, Day1, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        for (a_index, a) in input.iter().enumerate() {
            for (b_index, b) in input.iter().skip(a_index + 1).enumerate() {
                for (_c_index, c) in input.iter().skip(b_index + 1).enumerate() {
                    if a + b + c == 2020 {
                        return Ok(a * b * c);
                    }
                }
            }
        }
        Err(eyre::eyre!("no solution found"))
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_first_solution() {
    let input = r#"
    1721
    979
    366
    299
    675
    1456
    "#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day1, Part1>(input).unwrap(),
        514579
    );
}

#[test]
fn test_second_solution() {
    let input = r#"
    1721
    979
    366
    299
    675
    1456
    "#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day1, Part2>(input).unwrap(),
        241861950
    );
}
