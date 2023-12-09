//! Advent of code 2020 2
//!
//!! Link: <https://adventofcode.com/2020/day/2>
//!
//! Good luck!

use aoc::{parts::*, Solver};

use eyre::Context;
use eyre::ContextCompat;
use eyre::Report;

impl Solver<Year2020, Day2, Part1> for Solution {
    type Input<'a> = Vec<Row<'a>>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim())
            .map(Row::parse)
            .collect::<Result<_, _>>()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        Ok(input.iter().filter(|row| row.is_valid()).count())
    }
}

impl Solver<Year2020, Day2, Part2> for Solution {
    type Input<'a> = Vec<Row<'a>>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2020, Day2, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        Ok(input.iter().filter(|row| row.is_valid_official()).count())
    }
}

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    pub min: usize,
    pub max: usize,
    pub letter: char,
}

#[derive(Debug, PartialEq)]
pub struct Row<'a> {
    pub policy: PasswordPolicy,
    pub password: &'a str,
}

impl<'a> Row<'a> {
    pub fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.policy.letter {
                count += 1;
            }
        }
        count >= self.policy.min && count <= self.policy.max
    }
    pub fn is_valid_official(&self) -> bool {
        let mut count = 0;
        for (i, c) in self.password.chars().enumerate() {
            if c == self.policy.letter && (i + 1 == self.policy.min || i + 1 == self.policy.max) {
                count += 1;
            }
        }
        count == 1
    }
    pub fn parse(s: &'a str) -> Result<Self, Report> {
        let Some((policy, password)) = s.split_once(':') else {
            eyre::bail!("invalid row");
        };
        let Some((minmax, letter)) = policy.split_once(' ') else {
            eyre::bail!("invalid policy");
        };
        let (min, max) = minmax
            .split_once('-')
            .map(|(s1, s2)| -> Result<(_, _), Report> { Ok((s1.parse()?, s2.parse()?)) })
            .context("no -")?
            .context("couldn't parse numbers")?;

        let password = password.trim();
        let policy = PasswordPolicy {
            min,
            max,
            letter: letter.chars().next().unwrap(),
        };
        Ok(Row { policy, password })
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() {
    let input = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
    "#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day2, Part1>(input).unwrap(),
        2
    );
}

#[test]
fn test_solution_second() {
    let input = r#"
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    "#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day2, Part2>(input).unwrap(),
        1
    );
}
