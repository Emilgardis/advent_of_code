//! Advent of code 2022 3
//!
//!! Link: <https://adventofcode.com/2022/day/3>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2022, Day3, Part1> for Solution {
    type Input<'a> = Vec<&'a str>;

    type Output = u32;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.lines().collect())
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        let mut total_priority = 0;
        for line in input {
            let half = line.len() / 2;

            let mut common_chars = std::collections::BTreeSet::new();
            for ch in line[..half].chars() {
                if line[half..].contains(ch) {
                    common_chars.insert(ch);
                }
            }

            let priority = common_chars
                .iter()
                .map(|ch: &char| match ch {
                    'a'..='z' => ((*ch as u16) - 'a' as u16 + 1) as u32,
                    'A'..='Z' => ((*ch as u16) - 'A' as u16 + 1) as u32 + 26,
                    _ => panic!(),
                })
                .sum::<u32>();

            total_priority += priority;
        }
        Ok(total_priority)
    }
}

impl Solver<Year2022, Day3, Part2> for Solution {
    type Input<'a> = <Self as Solver<Year2022, Day3, Part1>>::Input<'a>;

    type Output = <Self as Solver<Year2022, Day3, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day3, Part1>>::generate_input(input)
    }

    fn solve(input: &Vec<&'_ str>) -> Result<Self::Output, Report> {
        let mut total_priority = 0;
        for line_3 in &input.iter().chunks(3) {
            let mut common_chars = std::collections::BTreeMap::new();
            for line in line_3 {
                for ch in line.chars().unique() {
                    *common_chars.entry(ch).or_insert(0) += 1;
                }
            }

            let priority = match common_chars
                .into_iter()
                .find(|(_, i)| *i == 3)
                .map(|(ch, i)| (ch, i))
            {
                Some((ch @ 'a'..='z', _)) => {
                    Ok::<_, eyre::Report>(((ch as u16) - 'a' as u16 + 1) as u32)
                }
                Some((ch @ 'A'..='Z', _)) => Ok(((ch as u16) - 'A' as u16 + 1) as u32 + 26),
                _ => eyre::bail!("wrong input"),
            }?;

            total_priority += priority;
        }
        Ok(total_priority)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day3, Part1>(input)?,
        157
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day3, Part2>(input)?,
        70
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day3, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day3, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
