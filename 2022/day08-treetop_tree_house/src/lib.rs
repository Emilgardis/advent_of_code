//! Advent of code 2022 8
//!
//!! Link: <https://adventofcode.com/2022/day/8>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a [tree house](https://en.wikipedia.org/wiki/Tree_house).
//!
//! First, determine whether there is enough tree cover here to keep a tree
//! house *hidden*. To do this, you need to count the number of trees that are
//! *visible from outside the grid* when looking directly along a row or column.
//!
//! The Elves have already launched a [quadcopter](https://en.wikipedia.org/wiki/Quadcopter) to generate a map with the height of each tree (your puzzle input). For example:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! Each tree is represented as a single digit whose value is its height, where
//! `0` is the shortest and `9` is the tallest.
//!
//! A tree is *visible* if all of the other trees between it and an edge of the
//! grid are *shorter* than it. Only consider trees in the same row or column;
//! that is, only look up, down, left, or right from any given tree.
//!
//! All of the trees around the edge of the grid are *visible* - since they are
//! already on the edge, there are no trees to block the view. In this example,
//! that only leaves the *interior nine trees* to consider:
//!
//! * The top-left `5` is *visible* from the left and top. (It isn't visible
//!   from the right or bottom since other trees of height `5` are in the way.)
//! * The top-middle `5` is *visible* from the top and right.
//! * The top-right `1` is not visible from any direction; for it to be visible,
//!   there would need to only be trees of height *0* between it and an edge.
//! * The left-middle `5` is *visible*, but only from the right.
//! * The center `3` is not visible from any direction; for it to be visible,
//!   there would need to be only trees of at most height `2` between it and an
//!   edge.
//! * The right-middle `3` is *visible* from the right.
//! * In the bottom row, the middle `5` is *visible*, but the `3` and `4` are
//!   not.
//!
//! With 16 trees visible on the edge and another 5 visible in the interior, a
//! total of `*21*` trees are visible in this arrangement.
//!
//! Consider your map; *how many trees are visible from outside the grid?*
//!
//! Your puzzle answer was `1688`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Content with the amount of tree cover available, the Elves just need to know
//! the best spot to build their tree house: they would like to be able to see a
//! lot of *trees*.
//!
//! To measure the viewing distance from a given tree, look up, down, left, and
//! right from that tree; stop if you reach an edge or at the first tree that is
//! the same height or taller than the tree under consideration. (If a tree is
//! right on the edge, at least one of its viewing distances will be zero.)
//!
//! The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large [eaves](https://en.wikipedia.org/wiki/Eaves) to keep it dry, so they wouldn't be able to see higher than the tree house anyway.
//!
//! In the example above, consider the middle `5` in the second row:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! * Looking up, its view is not blocked; it can see `*1*` tree (of height
//!   `3`).
//! * Looking left, its view is blocked immediately; it can see only `*1*` tree
//!   (of height `5`, right next to it).
//! * Looking right, its view is not blocked; it can see `*2*` trees.
//! * Looking down, its view is blocked eventually; it can see `*2*` trees (one
//!   of height `3`, then the tree of height `5` that blocks its view).
//!
//! A tree's *scenic score* is found by *multiplying together* its viewing
//! distance in each of the four directions. For this tree, this is `*4*` (found
//! by multiplying `1 * 1 * 2 * 2`).
//!
//! However, you can do even better: consider the tree of height `5` in the
//! middle of the fourth row:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! * Looking up, its view is blocked at `*2*` trees (by another tree with a
//!   height of `5`).
//! * Looking left, its view is not blocked; it can see `*2*` trees.
//! * Looking down, its view is also not blocked; it can see `*1*` tree.
//! * Looking right, its view is blocked at `*2*` trees (by a massive tree of
//!   height `9`).
//!
//! This tree's scenic score is `*8*` (`2 * 2 * 1 * 2`); this is the ideal spot
//! for the tree house.
//!
//! Consider each tree on your map. *What is the highest scenic score possible
//! for any tree?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](8/input).
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::{Context, Report};
use itertools::Itertools;

impl Solver<Year2022, Day8, Part1> for Solution {
    type Input<'a> = (usize, Vec<(u32, bool)>);

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok((
            input.lines().count(),
            input
                .lines()
                .map(|s| s.trim())
                .flat_map(|l| {
                    l.chars().map(|c| {
                        Ok::<_, eyre::Report>((
                            format!("{c}")
                                .parse::<u32>()
                                .wrap_err_with(|| eyre::eyre!("wtf: {c:?}"))?,
                            false,
                        ))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn solve(input: &(usize, Vec<(u32, bool)>)) -> Result<Self::Output, Report> {
        let mut vec = input.1.clone();
        let height = input.0;

        for y in 0..height {
            if y == 0 || y == height - 1 {
                for x in 0..height {
                    vec[y * height + x].1 = true;
                }
            }

            row(y, height, 0..height, &mut vec);
            row(y, height, (0..height).rev(), &mut vec);
        }
        for x in 0..height {
            if x == 0 || x == height - 1 {
                for y in 0..height {
                    vec[y * height + x].1 = true;
                }
            }

            // from up, down, left, right, check if visible.

            col(x, height, 0..height, &mut vec);
            col(x, height, (0..height).rev(), &mut vec);
        }
        println!(
            "\n{}",
            vec.iter()
                .chunks(height)
                .into_iter()
                .map(|a| a.map(|a| a.1 as usize).join(""))
                .join("\n")
        );
        Ok(vec.iter().filter(|(_, b)| *b).count())
    }
}

impl Solver<Year2022, Day8, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 8, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 8, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day8, Part1>>::generate_input(input)
    }

    fn solve(input: &(usize, Vec<(u32, bool)>)) -> Result<Self::Output, Report> {
        let mut vec = input.1.clone();
        let height = input.0;
        let mut max = 0;
        for y in 0..height {
            for x in 0..height {
                if x == 0 || y == 0 || x == height - 1 || y == height - 1 {
                    continue;
                }
                if y * height + x == 7 {
                    println!("fuxk")
                }
                vec[y * height + x].1 = true;
                let up = col(x, height, (dbg!(0..y)).rev(), &mut vec) + 1;
                let left = row(y, height, (dbg!(0..x)).rev(), &mut vec) + 1;
                let right = row(y, height, dbg!(x..height), &mut vec) + 1;
                let down = col(x, height, dbg!(y..height), &mut vec) + 1;
                let product = up * down * left * right;
                dbg!((x, y), up, left, right, down);
                if product > max {
                    max = product;
                }
            }
        }
        todo!("nope");
    }
}

// up, down
fn col(x: usize, len: usize, range: impl Iterator<Item = usize>, v: &mut [(u32, bool)]) -> usize {
    let mut max = (0, 0);
    for (i, y) in range.enumerate() {
        let th: u32 = v[y * len + x].0;
        if th <= max.0 {
            continue;
        } else {
            v[y * len + x].1 = true;
            if th > max.0 {
                max = (th, i + 1);
            }
        }
    }
    max.1
}

// left right
fn row(y: usize, len: usize, range: impl Iterator<Item = usize>, v: &mut [(u32, bool)]) -> usize {
    let mut max = (0, 0);
    for (i, x) in range.enumerate() {
        let th: u32 = v[y * len + x].0;
        if th <= max.0 {
            continue;
        } else {
            v[y * len + x].1 = true;
            if th > max.0 {
                max = (th, i + 1);
            }
        }
    }
    max.1
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
30373
25512
65332
33549
35390"#
        .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day8, Part1>(input)?,
        21
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
30373
25512
65332
33549
35390"#
        .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day8, Part2>(input)?,
        8
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day8, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day8, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
