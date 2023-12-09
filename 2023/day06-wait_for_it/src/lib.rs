//! Advent of code 2023 6
//!
//! Link: <https://adventofcode.com/2023/day/6>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

impl Solver<Year2023, Day6, Part1> for Solution {
    type Input<'a> = Vec<(u64, u64)>;

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut input = input.trim().lines();
        let time = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse());
        let distance = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse());
        time.zip(distance).map(|(a, b)| Ok((a?, b?))).collect()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        // find out the max time we can hold the button
        // the distance the boat travels is the formula
        // button_time * (allowed_time - button_time)
        // since we want to find the first time we can beat the record
        // we want to find the point
        // button_time * (allowed_time - button_time) = record_distance
        // if we expand the expression, we get
        // -button_time^2 + allowed_time * button_time = record_distance
        // which is a formula  like -x^2 + bx + c = 0
        // so we can solve it with the quadratic formula
        let mut prod = 1;
        for (a, r) in input {
            let a = *a as f64;
            let r = *r as f64;
            // let min_button_time = (-(*allowed_time as f64)
            //     + ((allowed_time.pow(2) + 4 * record_distance) as f64).sqrt())
            //     / 2.0;
            //     let max_button_time = (-(*allowed_time as f64)
            //     - ((allowed_time.pow(2) + 4 * record_distance) as f64).sqrt())
            //     / 2.0;
            let eps = 0.01;
            let left = (a.powi(2) - 4.0 * r).sqrt() / 2.0;
            let roots = [a / 2.0 - left + eps, a / 2.0 + left - eps];
            // this simplifies (ignoreing ceil and floor), can we use that fact?
            // floor(a/2 + left) - ceil(a/2 - left) + 1 =>
            // left*2 + 1
            // i'm not sure, the tricky part is that we need to offset by integer amounts
            // also
            let count = roots[1].trunc() as u64 - roots[0].ceil() as u64 + 1;
            //println!("roots: {roots:?} - count: {count}",);
            prod *= count;
        }
        Ok(prod)
    }
}

impl Solver<Year2023, Day6, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 6, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 6, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut input = input.trim().lines();
        let time = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()?;
        let distance = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()?;
        Ok(vec![(time, distance)])
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        <Self as Solver<Year2023, Day6, Part1>>::solve(input)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Time:      7  15   30
Distance:  9  40  200
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day6, Part1>(input)?,
        288
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Time:      7  15   30
Distance:  9  40  200
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day6, Part2>(input)?,
        71503
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day6, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day6, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
