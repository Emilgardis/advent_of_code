#![feature(extract_if)]
//! Advent of code 2021 3
//!
//!! Link: <https://adventofcode.com/2021/day/3>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2021, Day3, Part1> for Solution {
    /// Input is each line, not parsed in column order
    type Input<'a> = (usize, Vec<usize>);

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let len = input.lines().next().unwrap_or("").trim().len();
        Ok((
            len,
            input
                .lines()
                .map(|line| usize::from_str_radix(line.trim(), 2))
                .try_collect()?,
        ))
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut gamma = 0usize;
        let mut epsilon = 0usize;
        // Length of samples
        let sample_len = input.1.len();
        for index in 0..input.0 {
            // count zeroes in column
            let count = input
                .1
                .iter()
                .filter(|n| get_bit_at(**n as u32, index as u8))
                .count();
            gamma |= ((count > sample_len / 2) as usize) << index;
            epsilon |= ((count <= sample_len / 2) as usize) << index;
        }

        Ok(gamma * epsilon)
    }
}

/// Gets bit truthness at position
#[inline]
fn get_bit_at(input: u32, position: u8) -> bool {
    if position < 32 {
        input & (1 << position) != 0
    } else {
        false
    }
}

impl Solver<Year2021, Day3, Part2> for Solution {
    type Input<'a> = (usize, Vec<usize>);

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2021, Day3, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let _oxygen = 0usize;
        let _co2 = 0usize;
        let mut oxygen_items = input.1.clone();
        let mut co2_items = input.1.clone();

        for index in (0..input.0).rev() {
            // amount of zeroes

            if oxygen_items.len() > 1 {
                let count_ones = oxygen_items
                    .iter()
                    .filter(|n| get_bit_at(**n as u32, index as u8))
                    .count();
                let count_zeroes = oxygen_items.len() - count_ones;
                tracing::info!(
                    "got {count_ones} ones and {count_zeroes} zeroes. {} left",
                    oxygen_items.len()
                );

                let remove = if count_ones != count_zeroes {
                    count_ones <= oxygen_items.len() / 2
                } else {
                    false
                };
                tracing::info!("removing values that are {remove:?} for oxygen");
                let _ = oxygen_items.extract_if(|n| get_bit_at(*n as u32, index as u8) == remove);
                tracing::debug!(
                    "{}",
                    oxygen_items.iter().map(|n| format!("{n:b}")).join(" ")
                );
            }

            if co2_items.len() > 1 {
                let count_ones = co2_items
                    .iter()
                    .filter(|n| get_bit_at(**n as u32, index as u8))
                    .count();
                let count_zeroes = co2_items.len() - count_ones;
                //tracing::info!("got {count_ones} ones and {count_zeroes} zeroes");
                let remove = if count_ones != count_zeroes {
                    count_ones > co2_items.len() / 2
                } else {
                    true
                };
                //tracing::info!("removing values that are {remove:?} for co2");
                co2_items.extract_if(|n| get_bit_at(*n as u32, index as u8) == remove);
            }
        }

        if co2_items.len() != 1 || oxygen_items.len() != 1 {
            eyre::bail!("Too many items left");
        }

        Ok(co2_items[0] * oxygen_items[0])
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day3, Part1>(input)?,
        198
    );
    Ok(())
}

#[cfg(test)]
#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day3, Part2>(input)?,
        230
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day3, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day3, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
