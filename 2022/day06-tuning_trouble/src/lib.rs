#![feature(iter_array_chunks)]
//! Advent of code 2022 6
//!
//!! Link: <https://adventofcode.com/2022/day/6>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//! The preparations are finally complete; you and the Elves leave camp on foot
//! and begin to make your way toward the *star* fruit grove.
//!
//! As you move through the dense undergrowth, one of the Elves gives you a
//! handheld *device*. He says that it has many fancy features, but the most
//! important one to set up right now is the *communication system*.
//!
//! However, because he's heard you have [significant](/2016/day/6)
//! [experience](/2016/day/25) [dealing](/2019/day/7) [with](/2019/day/9)
//! [signal-based](/2019/day/16) [systems](/2021/day/25), he convinced the other
//! Elves that it would be okay to give you their one malfunctioning device -
//! surely you'll have no problem fixing it.
//!
//! As if inspired by comedic timing, the device emits a few colorful sparks.
//!
//! To be able to communicate with the Elves, the device needs to *lock on to
//! their signal*. The signal is a series of seemingly-random characters that
//! the device receives one at a time.
//!
//! To fix the communication system, you need to add a subroutine to the device
//! that detects a *start-of-packet marker* in the datastream. In the protocol
//! being used by the Elves, the start of a packet is indicated by a sequence of
//! *four characters that are all different*.
//!
//! The device will send your subroutine a datastream buffer (your puzzle
//! input); your subroutine needs to identify the first position where the four
//! most recently received characters were all different. Specifically, it needs
//! to report the number of characters from the beginning of the buffer to the
//! end of the first such four-character marker.
//!
//! For example, suppose you receive the following datastream buffer:
//!
//! ```
//! mjqjpqmgbljsphdztnvjfqwrcgsmlb
//! ```
//!
//! After the first three characters (`mjq`) have been received, there haven't
//! been enough characters received yet to find the marker. The first time a
//! marker could occur is after the fourth character is received, making the
//! most recent four characters `mjqj`. Because `j` is repeated, this isn't a
//! marker.
//!
//! The first time a marker appears is after the *seventh* character arrives.
//! Once it does, the last four characters received are `jpqm`, which are all
//! different. In this case, your subroutine should report the value `*7*`,
//! because the first start-of-packet marker is complete after 7 characters have
//! been processed.
//!
//! Here are a few more examples:
//!
//! * `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character `*5*`
//! * `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character `*6*`
//! * `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character `*10*`
//! * `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character `*11*`
//!
//! *How many characters need to be processed before the first start-of-packet
//! marker is detected?*
//!
//! To begin, [get your puzzle input](6/input).
//!
//! Answer:
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2022, Day6, Part1> for Solution {
    type Input<'a> = &'a str;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.trim())
    }

    fn solve(input: &&'_ str) -> Result<Self::Output, Report> {
        input
            .chars()
            .tuple_windows()
            .position(|(a, b, c, d)| [a, b, c, d].iter().all_unique())
            .ok_or_else(|| eyre::eyre!("none found"))
            .map(|p| p + 4)
    }
}

impl Solver<Year2022, Day6, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 6, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 6, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day6, Part1>>::generate_input(input)
    }

    fn solve(input: &&str) -> Result<Self::Output, Report> {
        let vec: Vec<_> = input.chars().collect();
        vec.windows(14)
            .position(|a| a.iter().all_unique())
            .ok_or_else(|| eyre::eyre!("none found"))
            .map(|p| p + 14)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>("bvwbjplbgvbhsrlpgdmjqwftvncz")?,
        5
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>("nppdvjthqldpwncqszvftbrmjlhg")?,
        6
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        )?,
        10
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part1>(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        )?,
        11
    );

    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();

    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?,
        19
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("bvwbjplbgvbhsrlpgdmjqwftvncz")?,
        23
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>("nppdvjthqldpwncqszvftbrmjlhg")?,
        23
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        )?,
        29
    );
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day6, Part2>(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        )?,
        26
    );

    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day6, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day6, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
