//! Advent of code 2023 2
//!
//! Link: <https://adventofcode.com/2023/day/2>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

pub struct Game {
    id: usize,
    cubes: Vec<Vec<(CubeColor, u32)>>,
}

#[derive(Debug, Clone, Copy)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl Solver<Year2023, Day2, Part1> for Solution {
    type Input<'a> = Vec<Game>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .lines()
            .map(|s| s.trim())
            .map(|l| {
                let Some((game, reveals)) = l.split_once(':') else {
                    eyre::bail!("invalid input, no : found")
                };
                let Some(id) = game.strip_prefix("Game ").map(|s| s.parse()).transpose()? else {
                    eyre::bail!("invalid input, no `Game ` found")
                };
                let cubes = reveals
                    .trim()
                    .split(';')
                    .map(|sets| {
                        sets.trim()
                            .split(',')
                            .map(|set| {
                                let (count, color) = set
                                    .trim()
                                    .split_once(' ')
                                    .ok_or_else(|| eyre::eyre!("invalid input, no space found"))?;
                                let count = count.parse()?;
                                let color = match color.trim() {
                                    "red" => CubeColor::Red,
                                    "green" => CubeColor::Green,
                                    "blue" => CubeColor::Blue,
                                    _ => eyre::bail!("invalid input, unknown color"),
                                };
                                Ok((color, count))
                            })
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>();

                Ok(Game { id, cubes: cubes? })
            })
            .collect::<Result<_, _>>()
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        'game: for game in input {
            for cubes in &game.cubes {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for (color, count) in cubes {
                    match color {
                        CubeColor::Red => red += count,
                        CubeColor::Green => green += count,
                        CubeColor::Blue => blue += count,
                    }
                }
                if red > 12 || green > 13 || blue > 14 {
                    println!("bad: {cubes:?}");
                    continue 'game;
                }
            }
            sum += game.id;
        }
        Ok(sum)
    }
}

impl Solver<Year2023, Day2, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 2, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 2, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day2, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut sum = 0;
        for game in input {
            let mut max_red = 0u32;
            let mut max_green = 0u32;
            let mut max_blue = 0u32;
            for cubes in &game.cubes {
                let mut red = 0u32;
                let mut green = 0u32;
                let mut blue = 0u32;
                for (color, count) in cubes {
                    match color {
                        CubeColor::Red => red += count,
                        CubeColor::Green => green += count,
                        CubeColor::Blue => blue += count,
                    }
                }
                if red > max_red {
                    max_red = red;
                }
                if green > max_green {
                    max_green = green;
                }
                if blue > max_blue {
                    max_blue = blue;
                }
            }

            sum += max_red * max_green * max_blue;
        }
        Ok(sum as usize)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day2, Part1>(input)?,
        8
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day2, Part2>(input)?,
        2286
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day2, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day2, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
