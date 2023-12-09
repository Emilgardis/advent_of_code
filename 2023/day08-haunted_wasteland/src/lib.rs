//! Advent of code 2023 8
//!
//! Link: <https://adventofcode.com/2023/day/8>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!You're still riding a camel across Desert Island when you spot a sandstorm
//! quickly approaching. When you turn to warn the Elf, she disappears before
//! your eyes! To be fair, she had just finished warning you about *ghosts* a
//! few minutes ago.
//!
//! One of the camel's pouches is labeled "maps" - sure enough, it's full of
//! documents (your puzzle input) about how to navigate the desert. At least,
//! you're pretty sure that's what they are; one of the documents contains a
//! list of left/right instructions, and the rest of the documents seem to
//! describe some kind of *network* of labeled nodes.
//!
//! It seems like you're meant to use the *left/right* instructions to *navigate
//! the network*. Perhaps if you have the camel follow the same instructions,
//! you can escape the haunted wasteland!
//!
//! After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`.
//! You feel like `AAA` is where you are now, and you have to follow the
//! left/right instructions until you reach `ZZZ`.
//!
//! This format defines each *node* of the network individually. For example:
//!
//! ```
//! RL
//!
//! AAA = (BBB, CCC)
//! BBB = (DDD, EEE)
//! CCC = (ZZZ, GGG)
//! DDD = (DDD, DDD)
//! EEE = (EEE, EEE)
//! GGG = (GGG, GGG)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting with `AAA`, you need to *look up the next element* based on the
//! next left/right instruction in your input. In this example, start with `AAA`
//! and go *right* (`R`) by choosing the right element of `AAA`, `*CCC*`. Then,
//! `L` means to choose the *left* element of `CCC`, `*ZZZ*`. By following the
//! left/right instructions, you reach `ZZZ` in `*2*` steps.
//!
//! Of course, you might not find `ZZZ` right away. If you run out of left/right
//! instructions, repeat the whole sequence of instructions as necessary: `RL`
//! really means `RLRLRLRLRLRLRLRL...` and so on. For example, here is a
//! situation that takes `*6*` steps to reach `ZZZ`:
//!
//! ```
//! LLR
//!
//! AAA = (BBB, BBB)
//! BBB = (AAA, ZZZ)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting at `AAA`, follow the left/right instructions. *How many steps are
//! required to reach `ZZZ`?*
//!
//! Your puzzle answer was `21251`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! The sandstorm is upon you and you aren't any closer to escaping the
//! wasteland. You had the camel follow the instructions, but you've barely left
//! your starting position. It's going to take *significantly more steps* to
//! escape!
//!
//! What if the map isn't for people - what if the map is for *ghosts*? Are
//! ghosts even bound by the laws of spacetime? Only one way to find out.
//!
//! After examining the maps a bit longer, your attention is drawn to a curious
//! fact: the number of nodes with names ending in `A` is equal to the number
//! ending in `Z`! If you were a ghost, you'd probably just *start at every node
//! that ends with `A`* and follow all of the paths at the same time until they
//! all simultaneously end up at nodes that end with `Z`.
//!
//! For example:
//!
//! ```
//! LR
//!
//! 11A = (11B, XXX)
//! 11B = (XXX, 11Z)
//! 11Z = (11B, XXX)
//! 22A = (22B, XXX)
//! 22B = (22C, 22C)
//! 22C = (22Z, 22Z)
//! 22Z = (22B, 22B)
//! XXX = (XXX, XXX)
//! ```
//!
//! Here, there are two starting nodes, `11A` and `22A` (because they both end
//! with `A`). As you follow each left/right instruction, use that instruction
//! to *simultaneously* navigate away from both nodes you're currently on.
//! Repeat this process until *all* of the nodes you're currently on end with
//! `Z`. (If only some of the nodes you're on end with `Z`, they act like any
//! other node and you continue as normal.) In this example, you would proceed
//! as follows:
//!
//! * Step 0: You are at `11A` and `22A`.
//! * Step 1: You choose all of the *left* paths, leading you to `11B` and
//!   `22B`.
//! * Step 2: You choose all of the *right* paths, leading you to `*11Z*` and
//!   `22C`.
//! * Step 3: You choose all of the *left* paths, leading you to `11B` and
//!   `*22Z*`.
//! * Step 4: You choose all of the *right* paths, leading you to `*11Z*` and
//!   `22B`.
//! * Step 5: You choose all of the *left* paths, leading you to `11B` and
//!   `22C`.
//! * Step 6: You choose all of the *right* paths, leading you to `*11Z*` and
//!   `*22Z*`.
//!
//! So, in this example, you end up entirely on nodes that end in `Z` after
//! `*6*` steps.
//!
//! Simultaneously start on every node that ends with `A`. *How many steps does
//! it take before you're only on nodes that end with `Z`?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](8/input).
//! <!---ENDOFDESCRIPTION--->
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
        for instruction in std::iter::repeat_with(|| map.instructions.iter()).flatten() {
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
        let mut steps = 0;
        println!("{map:?}");
        let mut current = map
            .map
            .keys()
            .filter(|&wp| wp.0.ends_with('A'))
            .collect_vec();
        for instruction in std::iter::repeat_with(|| map.instructions.iter()).flatten() {
            if current.iter().all(|&wp| wp.0.ends_with('Z')) {
                break;
            }
            for current in current.iter_mut() {
                let leftright = map
                    .map
                    .get(current)
                    .ok_or_else(|| eyre::eyre!("couldn't get lr: current: {current:?}"))?;
                *current = match &instruction {
                    b'L' => &leftright.left,
                    b'R' => &leftright.right,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };
            }
            steps += 1;
        }
        Ok(steps)
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
