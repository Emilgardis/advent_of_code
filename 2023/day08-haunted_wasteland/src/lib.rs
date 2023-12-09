//! Advent of code 2023 8
//!
//! Link: <https://adventofcode.com/2023/day/8>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::collections::BTreeMap;

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Waypoint<'a>(&'a str);

impl<'a> std::fmt::Debug for Waypoint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LeftRight<'a> {
    left: Waypoint<'a>,
    right: Waypoint<'a>,
}

impl<'a> std::fmt::Debug for LeftRight<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.left, self.right)
    }
}

pub struct Map<'a> {
    map: BTreeMap<Waypoint<'a>, LeftRight<'a>>,
    instructions: &'a [u8],
}

impl<'a> std::fmt::Debug for Map<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("map", &self.map)
            .field(
                "instructions",
                &std::str::from_utf8(self.instructions).unwrap(),
            )
            .finish()
    }
}

impl Solver<Year2023, Day8, Part1> for Solution {
    type Input<'a> = Map<'a>;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut lines = input.trim().lines();

        let instructions = lines
            .next()
            .ok_or_else(|| eyre::eyre!("no instructions"))?
            .as_bytes();
        let mut map = Map {
            map: BTreeMap::new(),
            instructions,
        };
        lines.next();
        lines.try_for_each(|s| {
            let Some((wp, leftright)) = s.split_once(" = ") else {
                eyre::bail!("no map found: {s}")
            };
            let Some((left, right)) = leftright.split_once(", ") else {
                eyre::bail!("invalid lr: {leftright}")
            };
            let left = left
                .strip_prefix('(')
                .ok_or_else(|| eyre::eyre!("invalid left"))?;
            let right = right
                .strip_suffix(')')
                .ok_or_else(|| eyre::eyre!("invalid right"))?;
            map.map.insert(
                Waypoint(wp),
                LeftRight {
                    left: Waypoint(left),
                    right: Waypoint(right),
                },
            );
            Ok(())
        })?;

        Ok(map)
    }

    fn solve(map: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut steps = 0;
        println!("{map:?}");
        let mut current = &Waypoint("AAA");
        for instruction in map.instructions.iter().cycle() {
            if current == &Waypoint("ZZZ") {
                break;
            }
            let leftright = map
                .map
                .get(current)
                .ok_or_else(|| eyre::eyre!("couldn't get lr: current: {current:?}"))?;
            current = match &instruction {
                b'L' => &leftright.left,
                b'R' => &leftright.right,
                _ => unreachable!(),
            };
            steps += 1;
        }
        Ok(steps)
    }
}

impl Solver<Year2023, Day8, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 8, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 8, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day8, Part1>>::generate_input(input)
    }

    fn solve(map: &Self::Input<'_>) -> Result<Self::Output, Report> {
        println!("{map:?}");
        let mut current = map
            .map
            .keys()
            .filter(|&wp| wp.0.ends_with('A'))
            .collect_vec();
        let mut counts = Vec::new();
        for current in current.iter_mut() {
            let mut count = 0i64;
            for instruction in map.instructions.iter().cycle() {
                if current.0.ends_with('Z') {
                    break;
                }
                let leftright = map
                    .map
                    .get(current)
                    .ok_or_else(|| eyre::eyre!("couldn't get lr: current: {current:?}"))?;
                *current = match &instruction {
                    b'L' => &leftright.left,
                    b'R' => &leftright.right,
                    _ => unreachable!(),
                };
                count += 1;
            }
            counts.push(count);
        }
        Ok(counts.into_iter().reduce(lcm).unwrap() as usize)
    }
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day8, Part1>(input)?,
        6
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day8, Part2>(input)?,
        6
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day8, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day8, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
