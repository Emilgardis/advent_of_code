//! Advent of code 2021 2
//!
//!! Link: <https://adventofcode.com/2021/day/2>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::collections::HashMap;

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq)]
pub enum Move {
    Forward,
    Down,
    Up,
}

pub struct MoveSet(Move, i32);

impl std::str::FromStr for MoveSet {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s
                .split_once(' ')
                .ok_or_else(|| eyre::eyre!("Invalid move: {}", s))
                .and_then(|(m, i)| Ok((m, i.parse()?)))?
            {
                ("forward", i) => MoveSet(Move::Forward, i),
                ("down", i) => MoveSet(Move::Down, i),
                ("up", i) => MoveSet(Move::Up, i),
                _ => eyre::bail!("Invalid move: {}", s),
            },
        )
    }
}

impl Solver<Year2021, Day2, Part1> for Solution {
    type Input<'a> = Vec<MoveSet>;

    type Output = i32;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .lines()
            .map(|line| line.trim().parse::<MoveSet>())
            .try_collect()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut set = HashMap::new();
        for m in input {
            let count = set.entry(&m.0).or_insert(0);
            *count += m.1;
        }

        let depth = set.get(&Move::Down).cloned().unwrap_or_default()
            - set.get(&Move::Up).cloned().unwrap_or_default();
        let forward = set.get(&Move::Forward).cloned().unwrap_or_default();
        Ok(depth * forward)
    }
}

impl Solver<Year2021, Day2, Part2> for Solution {
    type Input<'a> = Vec<MoveSet>;

    type Output = i32;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2021, Day2, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut aim = 0;
        let mut depth = 0;
        let mut forward = 0;
        for m in input {
            match m.0 {
                Move::Down => aim += m.1,
                Move::Up => aim -= m.1,
                Move::Forward => {
                    forward += m.1;
                    depth += aim * m.1;
                }
            }
        }
        Ok(depth * forward)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day2, Part1>(input)?,
        150
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day2, Part2>(input)?,
        900
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day2, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day2, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
