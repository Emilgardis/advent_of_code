#![feature(generic_associated_types)]
//! Advent of code 2020 1
//!
//! https://adventofcode.com/2020/day/1
//!
//! ## Description
//!
//! \--- Day 1: Report Repair ---
//! ----------
//!
//! After saving Christmas [five years in a row](/events), you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them *stars*. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all *fifty stars* by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your *expense report* (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to *find the two entries that sum to `2020`* and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//!
//! ```
//!
//! In this list, the two entries that sum to `2020` are `1721` and `299`. Multiplying them together produces `1721 * 299 = 514579`, so the correct answer is `*514579*`.
//!
//! Of course, your expense report is much larger. *Find the two entries that sum to `2020`; what do you get if you multiply them together?*
//!
//! To begin, [get your puzzle input](1/input).
//!
//! Answer:

use aoc::Aoc;
use aoc::Solver;
use eyre::Report;
use itertools::Itertools;

impl Solver for Solution {
    type Input<'a> = Vec<usize>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input
            .split_whitespace()
            .map(|i| i.parse())
            .collect::<Result<_, _>>()?)
    }

    fn solve_first(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        for (a_index, a) in input.iter().enumerate() {
            for b in input.iter().skip(a_index + 1) {
                if a + b == 2020 {
                    return Ok(a * b);
                }
            }
        }
        Err(eyre::eyre!("no solution found"))
    }

    fn solve_second(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        for (a_index, a) in input.iter().enumerate() {
            for (b_index, b) in input.iter().skip(a_index + 1).enumerate() {
                for (c_index, c) in input.iter().skip(b_index + 1).enumerate() {
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
    let input = Solution::generate_input(input).unwrap();
    assert_eq!(Solution::solve_first(&input).unwrap(), 514579);
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
    let input = Solution::generate_input(input).unwrap();
    assert_eq!(Solution::solve_second(&input).unwrap(), 241861950);
}
