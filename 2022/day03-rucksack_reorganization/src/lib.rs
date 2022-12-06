//! Advent of code 2022 3
//!
//! https://adventofcode.com/2022/day/3
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!One Elf has the important job of loading all of the [rucksacks](https://en.wikipedia.org/wiki/Rucksack) with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//!
//! Each rucksack has two large *compartments*. All items of a given type are
//! meant to go into exactly one of the two compartments. The Elf that did the
//! packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack
//! (your puzzle input), but they need your help finding the errors. Every item
//! type is identified by a single lowercase or uppercase letter (that is, `a`
//! and `A` refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single
//! line. A given rucksack always has the same number of items in each of its
//! two compartments, so the first half of the characters represent items in the
//! first compartment, while the second half of the characters represent items
//! in the second compartment.
//!
//! For example, suppose you have the following list of contents from six
//! rucksacks:
//!
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! * The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which
//!   means its first compartment contains the items `vJrwpWtwJgWr`, while the
//!   second compartment contains the items `hcsFMMfFFhFp`. The only item type
//!   that appears in both compartments is lowercase `*p*`.
//! * The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and
//!   `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments
//!   is uppercase `*L*`.
//! * The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the
//!   only common item type is uppercase `*P*`.
//! * The fourth rucksack's compartments only share item type `*v*`.
//! * The fifth rucksack's compartments only share item type `*t*`.
//! * The sixth rucksack's compartments only share item type `*s*`.
//!
//! To help prioritize item rearrangement, every item type can be converted to a
//! *priority*:
//!
//! * Lowercase item types `a` through `z` have priorities 1 through 26.
//! * Uppercase item types `A` through `Z` have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both
//! compartments of each rucksack is 16 (`p`), 38 (`L`), 42 (`P`), 22 (`v`), 20
//! (`t`), and 19 (`s`); the sum of these is `*157*`.
//!
//! Find the item type that appears in both compartments of each rucksack. *What
//! is the sum of the priorities of those item types?*
//!
//! Your puzzle answer was `8401`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! As you finish identifying the misplaced items, the Elves come to you with
//! another issue.
//!
//! For safety, the Elves are divided into groups of three. Every Elf carries a
//! badge that identifies their group. For efficiency, within each group of
//! three Elves, the badge is the *only item type carried by all three Elves*.
//! That is, if a group's badge is item type `B`, then all three Elves will have
//! item type `B` somewhere in their rucksack, and at most two of the Elves will
//! be carrying any other item type.
//!
//! The problem is that someone forgot to put this year's updated authenticity
//! sticker on the badges. All of the badges need to be pulled out of the
//! rucksacks so the new authenticity stickers can be attached.
//!
//! Additionally, nobody wrote down which item type corresponds to each group's
//! badges. The only way to tell which item type is the right one is by finding
//! the one item type that is *common between all three Elves* in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but
//! each group can have a different badge item type. So, in the above example,
//! the first group's rucksacks are the first three lines:
//!
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! ```
//!
//! And the second group's rucksacks are the next three lines:
//!
//! ```
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! In the first group, the only item type that appears in all three rucksacks
//! is lowercase `r`; this must be their badges. In the second group, their
//! badge item type must be `Z`.
//!
//! Priorities for these items must still be found to organize the sticker
//! attachment efforts: here, they are 18 (`r`) for the first group and 52 (`Z`)
//! for the second group. The sum of these is `*70*`.
//!
//! Find the item type that corresponds to the badges of each three-Elf group.
//! *What is the sum of the priorities of those item types?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](3/input).
//! <!---ENDOFDESCRIPTION--->
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
