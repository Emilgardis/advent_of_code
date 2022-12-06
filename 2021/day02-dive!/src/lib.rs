//! Advent of code 2021 2
//!
//! https://adventofcode.com/2021/day/2
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!\--- Day 2: Dive! ---
//! ----------
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like `forward 1`,
//! `down 2`, or `up 3`:
//!
//! * `forward X` increases the horizontal position by `X` units.
//! * `down X` *increases* the depth by `X` units.
//! * `up X` *decreases* the depth by `X` units.
//!
//! Note that since you're on a submarine, `down` and `up` affect your *depth*,
//! and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! ```
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at `0`. The steps above would
//! then modify them as follows:
//!
//! * `forward 5` adds `5` to your horizontal position, a total of `5`.
//! * `down 5` adds `5` to your depth, resulting in a value of `5`.
//! * `forward 8` adds `8` to your horizontal position, a total of `13`.
//! * `up 3` decreases your depth by `3`, resulting in a value of `2`.
//! * `down 8` adds `8` to your depth, resulting in a value of `10`.
//! * `forward 2` adds `2` to your horizontal position, a total of `15`.
//!
//! After following these instructions, you would have a horizontal position of
//! `15` and a depth of `10`. (Multiplying these together produces `*150*`.)
//!
//! Calculate the horizontal position and depth you would have after following
//! the planned course. *What do you get if you multiply your final horizontal
//! position by your final depth?*
//!
//! Your puzzle answer was `2019945`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Based on your calculations, the planned course doesn't seem to make any
//! sense. You find the submarine manual and discover that the process is
//! actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a
//! third value, *aim*, which also starts at `0`. The commands also mean
//! something entirely different than you first thought:
//!
//! * `down X` *increases* your aim by `X` units.
//! * `up X` *decreases* your aim by `X` units.
//! * `forward X` does two things:
//!   * It increases your horizontal position by `X` units.
//!   * It increases your depth by your aim *multiplied by* `X`.
//!
//! Again note that since you're on a submarine, `down` and `up` do the opposite
//! of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! * `forward 5` adds `5` to your horizontal position, a total of `5`. Because
//!   your aim is `0`, your depth does not change.
//! * `down 5` adds `5` to your aim, resulting in a value of `5`.
//! * `forward 8` adds `8` to your horizontal position, a total of `13`. Because
//!   your aim is `5`, your depth increases by `8*5=40`.
//! * `up 3` decreases your aim by `3`, resulting in a value of `2`.
//! * `down 8` adds `8` to your aim, resulting in a value of `10`.
//! * `forward 2` adds `2` to your horizontal position, a total of `15`. Because
//!   your aim is `10`, your depth increases by `2*10=20` to a total of `60`.
//!
//! After following these new instructions, you would have a horizontal position
//! of `15` and a depth of `60`. (Multiplying these produces `*900*`.)
//!
//! Using this new interpretation of the commands, calculate the horizontal
//! position and depth you would have after following the planned course. *What
//! do you get if you multiply your final horizontal position by your final
//! depth?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](2/input).
//! <!---ENDOFDESCRIPTION--->
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
