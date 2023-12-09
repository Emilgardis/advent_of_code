//! Advent of code 2023 1
//!
//! Link: <https://adventofcode.com/2023/day/1>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2023, Day1, Part1> for Solution {
    type Input<'a> = Vec<&'a str>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.lines().map(|s| s.trim()).collect())
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for line in input {
            let Some(first) = line.chars().find_map(|c| c.to_digit(10)) else {
                eyre::bail!("oops no match")
            };
            let Some(last) = line.chars().rev().find_map(|c| c.to_digit(10)) else {
                eyre::bail!("oops no match")
            };
            sum += (first * 10) as usize + last as usize;
        }
        Ok(sum)
    }
}

impl Solver<Year2023, Day1, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 1, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 1, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day1, Part1>>::generate_input(input)
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        const NUMBERS: &[(&'static str, usize)] = &[
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        for &line in input {
            let first = 'first: {
                let first_digit = line
                    .char_indices()
                    .find_map(|(i, c)| Some((i, c.to_digit(10)?)));
                let (first, _) = if let Some(first_digit) = first_digit {
                    line.split_at(first_digit.0)
                } else {
                    (line, "")
                };
                for i in 0..first.len() {
                    for word in NUMBERS {
                        if first[i..].starts_with(word.0) {
                            break 'first word.1;
                        }
                    }
                }
                first_digit.unwrap().1 as usize
            };
            println!("first {first}");
            let last = 'last: {
                let last_digit = line
                    .char_indices()
                    .rev()
                    .find_map(|(i, c)| Some((i, c.to_digit(10)?)));
                let (_, last) = if let Some(last_digit) = last_digit {
                    line.split_at(last_digit.0)
                } else {
                    ("", line)
                };
                for i in 0..last.len() {
                    for word in NUMBERS {
                        if last[..last.len() - i].ends_with(word.0) {
                            break 'last word.1;
                        }
                    }
                }
                last_digit.unwrap().1 as usize
            };
            println!("last {last}");
            sum += (first * 10) as usize + last;
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
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day1, Part1>(input)?,
        142
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day1, Part2>(input)?,
        281
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day1, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day1, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
