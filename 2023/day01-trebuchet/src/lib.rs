//! Advent of code 2023 1
//!
//! Link: <https://adventofcode.com/2023/day/1>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!Something is wrong with global snow production, and you've been selected to
//! take a look. The Elves have even given you a map; on it, they've used stars
//! to mark the top fifty locations that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations,
//! you need to check all *fifty stars* by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the Advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants *one star*. Good luck!
//!
//! You try to ask why they can't just use a [weather machine](/2015/day/1) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a [trebuchet](https://en.wikipedia.org/wiki/Trebuchet) ("please hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their
//! calibration document (your puzzle input) has been *amended* by a very young
//! Elf who was apparently just excited to show off her art skills.
//! Consequently, the Elves are having trouble reading the values on the
//! document.
//!
//! The newly-improved calibration document consists of lines of text; each line
//! originally contained a specific *calibration value* that the Elves now need
//! to recover. On each line, the calibration value can be found by combining
//! the *first digit* and the *last digit* (in that order) to form a single
//! *two-digit number*.
//!
//! For example:
//!
//! ```
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//! ```
//!
//! In this example, the calibration values of these four lines are `12`, `38`,
//! `15`, and `77`. Adding these together produces `*142*`.
//!
//! Consider your entire calibration document. *What is the sum of all of the
//! calibration values?*
//!
//! Your puzzle answer was `57346`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Your calculation isn't quite right. It looks like some of the digits are
//! actually *spelled out with letters*: `one`, `two`, `three`, `four`, `five`,
//! `six`, `seven`, `eight`, and `nine` *also* count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and
//! last digit on each line. For example:
//!
//! ```
//! two1nine
//! eightwothree
//! abcone2threexyz
//! xtwone3four
//! 4nineeightseven2
//! zoneight234
//! 7pqrstsixteen
//! ```
//!
//! In this example, the calibration values are `29`, `83`, `13`, `24`, `42`,
//! `14`, and `76`. Adding these together produces `*281*`.
//!
//! *What is the sum of all of the calibration values?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](1/input).
//! <!---ENDOFDESCRIPTION--->
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
