//! Advent of code 2022 4
//!
//! https://adventofcode.com/2022/day/4
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique *ID number*, and each Elf is assigned a range of section IDs.
//!
//! However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments *overlap*. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a *big list of the section assignments for each pair* (your puzzle input).
//!
//! For example, consider the following list of section assignment pairs:
//!
//! ```
//! 2-4,6-8
//! 2-3,4-5
//! 5-7,7-9
//! 2-8,3-7
//! 6-6,4-6
//! 2-6,4-8
//!
//! ```
//!
//! For the first few pairs, this list means:
//!
//! * Within the first pair of Elves, the first Elf was assigned sections `2-4` (sections `2`, `3`, and `4`), while the second Elf was assigned sections `6-8` (sections `6`, `7`, `8`).
//! * The Elves in the second pair were each assigned two sections.
//! * The Elves in the third pair were each assigned three sections: one got sections `5`, `6`, and `7`, while the other also got `7`, plus `8` and `9`.
//!
//! This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
//!
//! ```
//! .234.....  2-4
//! .....678.  6-8
//!
//! .23......  2-3
//! ...45....  4-5
//!
//! ....567..  5-7
//! ......789  7-9
//!
//! .2345678.  2-8
//! ..34567..  3-7
//!
//! .....6...  6-6
//! ...456...  4-6
//!
//! .23456...  2-6
//! ...45678.  4-8
//!
//! ```
//!
//! Some of the pairs have noticed that one of their assignments *fully contains* the other. For example, `2-8` fully contains `3-7`, and `6-6` is fully contained by `4-6`. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are `*2*` such pairs.
//!
//! *In how many assignment pairs does one range fully contain the other?*
//!
//! Your puzzle answer was `576`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that *overlap at all*.
//!
//! In the above example, the first two pairs (`2-4,6-8` and `2-3,4-5`) don't overlap, while the remaining four pairs (`5-7,7-9`, `2-8,3-7`, `6-6,4-6`, and `2-6,4-8`) do overlap:
//!
//! * `5-7,7-9` overlaps in a single section, `7`.
//! * `2-8,3-7` overlaps all of the sections `3` through `7`.
//! * `6-6,4-6` overlaps in a single section, `6`.
//! * `2-6,4-8` overlaps in sections `4`, `5`, and `6`.
//!
//! So, in this example, the number of overlapping assignment pairs is `*4*`.
//!
//! *In how many assignment pairs do the ranges overlap?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](4/input).
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use std::collections::BTreeSet;

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
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day4, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}
