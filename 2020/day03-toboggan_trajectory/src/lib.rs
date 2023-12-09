//! Advent of code 2020 3
//!
//!! Link: <https://adventofcode.com/2020/day/3>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;

const SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

#[derive(Debug)]
pub struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    pub const fn new(right: usize, down: usize) -> Self {
        Self { right, down }
    }
}

#[tracing::instrument()]
pub fn check_slope(slope: &Slope, map: &[&str]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < map.len() {
        if map[y].chars().nth(x) == Some('#') {
            trees += 1;
        }
        // There is an infinite amount of repeating tiles to the right, this makes sure
        // x doesn't go outside the range
        x = (x + slope.right) % map[y].len();
        y += slope.down;
    }
    trees
}

impl Solver<Year2020, Day3, Part1> for Solution {
    type Input<'a> = Vec<&'a str>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(input.lines().map(|l| l.trim()).collect())
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        Ok(check_slope(&Slope::new(3, 1), input))
    }
}

impl Solver<Year2020, Day3, Part2> for Solution {
    type Input<'a> = Vec<&'a str>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2020, Day3, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut total = 1;
        for slope in SLOPES {
            total *= check_slope(slope, input);
        }
        Ok(total)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day3, Part1>(input)?,
        7
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2020, Day3, Part2>(input)?,
        336
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2020, Day3, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2020, Day3, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
