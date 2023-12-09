#![feature(iter_array_chunks)]
//! Advent of code 2023 5
//!
//! Link: <https://adventofcode.com/2023/day/5>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::ops::Range;

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

#[derive(Debug)]
pub struct MapLine {
    range: Range<u64>,
    offset: i64,
}

impl MapLine {
    pub fn new(line: &str) -> Result<Self, Report> {
        let (destination_range_start, source_range_start, range_length) = line
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| eyre::eyre!("couldn't collect numbers"))?;
        let destination_range_start: u64 = destination_range_start.parse()?;
        let source_range_start: u64 = source_range_start.parse()?;
        let range_length: u64 = range_length.parse()?;
        Ok(MapLine {
            range: source_range_start..source_range_start + range_length,
            offset: (destination_range_start as i64) - (source_range_start as i64),
        })
    }

    /// Translate a number from the source range to the destination range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use y2023_day5_if_you_give_a_seed_a_fertilizer::MapLine;
    /// let map_line = MapLine::new("50 98 2").unwrap();
    /// assert_eq!(map_line.translate(98), Some(50));
    /// assert_eq!(map_line.translate(99), Some(51));
    /// assert_eq!(map_line.translate(100), None);
    /// assert_eq!(map_line.translate(97), None);
    /// ```
    ///
    /// ```rust
    /// # use y2023_day5_if_you_give_a_seed_a_fertilizer::MapLine;
    /// let map_line = MapLine::new("52 50 48").unwrap();
    /// assert_eq!(map_line.translate(50), Some(52));
    /// assert_eq!(map_line.translate(51), Some(53));
    /// assert_eq!(map_line.translate(52), Some(54));
    /// assert_eq!(map_line.translate(53), Some(55));
    /// assert_eq!(map_line.translate(96), None);
    /// assert_eq!(map_line.translate(97), None);
    /// assert_eq!(map_line.translate(98), None);
    /// ````
    ///
    /// ```rust
    /// # use y2023_day5_if_you_give_a_seed_a_fertilizer::MapLine;
    /// let map_line = MapLine::new("49 53 8").unwrap();
    /// assert_eq!(map_line.translate(53), Some(49));
    /// ```
    pub fn translate(&self, from: u64) -> Option<u64> {
        if self.range.contains(&from) {
            Some((from as i64 + self.offset) as u64)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Map<'a> {
    from: &'a str,
    to: &'a str,
    lines: Vec<MapLine>,
}

impl<'a> Map<'a> {
    pub fn new(s: &'a str) -> Result<Self, Report> {
        let mut lines = s.lines();
        let (name, _) = lines
            .next()
            .unwrap()
            .split_once(" map:")
            .ok_or_else(|| eyre::eyre!("couldn't split on ` map:`"))?;
        let (from, to) = name
            .split_once("-to-")
            .ok_or_else(|| eyre::eyre!("couldn't split on `-to-`"))?;

        Ok(Map {
            from,
            to,
            lines: lines.map(MapLine::new).try_collect()?,
        })
    }

    fn translate(&self, seed: u64) -> u64 {
        self.lines
            .iter()
            .find_map(|line| line.translate(seed))
            .unwrap_or(seed)
    }
}

impl Solver<Year2023, Day5, Part1> for Solution {
    type Input<'a> = (Vec<u64>, Vec<Map<'a>>);

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let (seeds, maps) = input
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| eyre::eyre!("couldn't split on first line"))?;
        let seeds = seeds
            .split_whitespace()
            .skip(1)
            .map(|seed| seed.parse())
            .try_collect()?;
        let maps = maps
            .trim()
            .split("\n\n")
            .map(|s| s.trim())
            .map(Map::new)
            .try_collect()?;
        Ok((seeds, maps))
    }

    fn solve((seeds, maps): &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut lowest = None;
        for seed in seeds {
            let mut seed = *seed;
            for map in maps {
                seed = map.translate(seed);
            }
            if lowest.is_none() || seed < lowest.unwrap() {
                lowest = Some(seed);
            }
        }
        Ok(lowest.unwrap())
    }
}

impl Solver<Year2023, Day5, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 5, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 5, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let (seeds, maps) = input
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| eyre::eyre!("couldn't split on first line"))?;
        let seeds = seeds
            .split_whitespace()
            .skip(1)
            .map(|seed| seed.parse().expect("couldn't parse"))
            .array_chunks()
            .fuse()
            .flat_map(|[start, range]: [u64; 2]| start..(start + range))
            .collect();
        let maps = maps
            .trim()
            .split("\n\n")
            .map(|s| s.trim())
            .map(Map::new)
            .try_collect()?;
        Ok((seeds, maps))
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        <Self as Solver<Year2023, Day5, Part1>>::solve(input)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day5, Part1>(input)?,
        35
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day5, Part2>(input)?,
        46
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day5, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day5, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
