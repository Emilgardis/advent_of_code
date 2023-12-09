//! Advent of code 2022 8
//!
//!! Link: <https://adventofcode.com/2022/day/8>
//!
//! Good luck!
//!
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
