//! Advent of code 2022 1
//!
//!! Link: <https://adventofcode.com/2022/day/1>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::{Context, Report};

impl Solver<Year2022, Day1, Part1> for Solution {
    type Input<'a> = Vec<Vec<usize>>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .trim()
            .split("\n\n")
            .map(|s| {
                s.split('\n')
                    .map(|l| {
                        l.parse()
                            .wrap_err_with(|| format!("couldn't parse line: {l:?}"))
                    })
                    .collect::<Result<_, _>>()
            })
            .collect()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        input
            .iter()
            .map(|v| v.iter().sum())
            .max()
            .ok_or_else(|| eyre::eyre!("lol"))
    }
}

impl Solver<Year2022, Day1, Part2> for Solution {
    type Input<'a> = Vec<Vec<usize>>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day1, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sums = input.iter().map(|v| v.iter().sum()).collect::<Vec<_>>();
        sums.sort();
        sums.reverse();

        Ok(sums.iter().take(3).sum())
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day1, Part1>(input)?,
        24000
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day1, Part2>(input)?,
        45000
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day1, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day1, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
