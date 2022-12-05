//! Advent of code 2020 3
//!
//! https://adventofcode.com/2020/day/3
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!\--- Day 3: Toboggan Trajectory ---
//! ----------
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (`.`) and trees (`#`) you can see. For example:
//!
//! ```
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//!
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ```
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//!
//! ```
//!
//! You start on the open square (`.`) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by *counting all the trees* you would encounter for the slope *right 3, down 1*:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with `*O*` where there was an open square and `*X*` where there was a tree:
//!
//! ```
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//!
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter `*7*` trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, *how many trees would you encounter?*
//!
//! Your puzzle answer was `270`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! * Right 1, down 1.
//! * Right 3, down 1. (This is the slope you already checked.)
//! * Right 5, down 1.
//! * Right 7, down 1.
//! * Right 1, down 2.
//!
//! In the above example, these slopes would find `2`, `7`, `3`, `4`, and `2` tree(s) respectively; multiplied together, these produce the answer `*336*`.
//!
//! *What do you get if you multiply together the number of trees encountered on each of the listed slopes?*
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
        // There is an infinite amount of repeating tiles to the right, this makes sure x doesn't go outside the range
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
