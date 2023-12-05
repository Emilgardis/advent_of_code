#![feature(iter_array_chunks)]
//! Advent of code 2023 5
//!
//! Link: <https://adventofcode.com/2023/day/5>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!You take the boat and find the gardener right where you were told he would
//! be: managing a giant "garden" that looks more to you like a farm.
//!
//! "A water source? Island Island *is* the water source!" You point out that
//! Snow Island isn't receiving any water.
//!
//! "Oh, we had to stop the water because we *ran out of sand* to [filter](https://en.wikipedia.org/wiki/Sand_filter) it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
//!
//! "I've been so busy making sure everyone here has food that I completely
//! forgot to check why we stopped getting more sand! There's a ferry leaving
//! soon that is headed over in that direction - it's much faster than your
//! boat. Could you please go check it out?"
//!
//! You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our *food production problem*. The latest Island Island [Almanac](https://en.wikipedia.org/wiki/Almanac) just arrived and we're having trouble making sense of it."
//!
//! The almanac (your puzzle input) lists all of the seeds that need to be
//! planted. It also lists what type of soil to use with each kind of seed, what
//! type of fertilizer to use with each kind of soil, what type of water to use
//! with each kind of fertilizer, and so on. Every type of seed, soil,
//! fertilizer and so on is identified with a number, but numbers are reused by
//! each category - that is, soil `123` and fertilizer `123` aren't necessarily
//! related to each other.
//!
//! For example:
//!
//! ```
//! seeds: 79 14 55 13
//!
//! seed-to-soil map:
//! 50 98 2
//! 52 50 48
//!
//! soil-to-fertilizer map:
//! 0 15 37
//! 37 52 2
//! 39 0 15
//!
//! fertilizer-to-water map:
//! 49 53 8
//! 0 11 42
//! 42 0 7
//! 57 7 4
//!
//! water-to-light map:
//! 88 18 7
//! 18 25 70
//!
//! light-to-temperature map:
//! 45 77 23
//! 81 45 19
//! 68 64 13
//!
//! temperature-to-humidity map:
//! 0 69 1
//! 1 0 69
//!
//! humidity-to-location map:
//! 60 56 37
//! 56 93 4
//! ```
//!
//! The almanac starts by listing which seeds need to be planted: seeds `79`,
//! `14`, `55`, and `13`.
//!
//! The rest of the almanac contains a list of *maps* which describe how to
//! convert numbers from a *source category* into numbers in a *destination
//! category*. That is, the section that starts with `seed-to-soil map:`
//! describes how to convert a *seed number* (the source) to a *soil number*
//! (the destination). This lets the gardener and his team know which soil to
//! use with which seeds, which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination
//! number one by one, the maps describe entire *ranges* of numbers that can be
//! converted. Each line within a map contains three numbers: the *destination
//! range start*, the *source range start*, and the *range length*.
//!
//! Consider again the example `seed-to-soil map`:
//!
//! ```
//! 50 98 2
//! 52 50 48
//! ```
//!
//! The first line has a *destination range start* of `50`, a *source range
//! start* of `98`, and a *range length* of `2`. This line means that the source
//! range starts at `98` and contains two values: `98` and `99`. The destination
//! range is the same length, but it starts at `50`, so its two values are `50`
//! and `51`. With this information, you know that seed number `98` corresponds
//! to soil number `50` and that seed number `99` corresponds to soil number
//! `51`.
//!
//! The second line means that the source range starts at `50` and contains `48`
//! values: `50`, `51`, ..., `96`, `97`. This corresponds to a destination range
//! starting at `52` and also containing `48` values: `52`, `53`, ..., `98`,
//! `99`. So, seed number `53` corresponds to soil number `55`.
//!
//! Any source numbers that *aren't mapped* correspond to the *same* destination
//! number. So, seed number `10` corresponds to soil number `10`.
//!
//! So, the entire list of seed numbers and their corresponding soil numbers
//! looks like this:
//!
//! ```
//! seed  soil
//! 0     0
//! 1     1
//! ...   ...
//! 48    48
//! 49    49
//! 50    52
//! 51    53
//! ...   ...
//! 96    98
//! 97    99
//! 98    50
//! 99    51
//! ```
//!
//! With this map, you can look up the soil number required for each initial
//! seed number:
//!
//! * Seed number `79` corresponds to soil number `81`.
//! * Seed number `14` corresponds to soil number `14`.
//! * Seed number `55` corresponds to soil number `57`.
//! * Seed number `13` corresponds to soil number `13`.
//!
//! The gardener and his team want to get started as soon as possible, so they'd
//! like to know the closest location that needs a seed. Using these maps, find
//! *the lowest location number that corresponds to any of the initial seeds*.
//! To do this, you'll need to convert each seed number through other categories
//! until you can find its corresponding *location number*. In this example, the
//! corresponding types are:
//!
//! * Seed `79`, soil `81`, fertilizer `81`, water `81`, light `74`, temperature
//!   `78`, humidity `78`, *location `82`*.
//! * Seed `14`, soil `14`, fertilizer `53`, water `49`, light `42`, temperature
//!   `42`, humidity `43`, *location `43`*.
//! * Seed `55`, soil `57`, fertilizer `57`, water `53`, light `46`, temperature
//!   `82`, humidity `82`, *location `86`*.
//! * Seed `13`, soil `13`, fertilizer `52`, water `41`, light `34`, temperature
//!   `34`, humidity `35`, *location `35`*.
//!
//! So, the lowest location number in this example is `*35*`.
//!
//! *What is the lowest location number that corresponds to any of the initial
//! seed numbers?*
//!
//! Your puzzle answer was `84470622`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Everyone will starve if you only plant such a small number of seeds.
//! Re-reading the almanac, it looks like the `seeds:` line actually describes
//! *ranges of seed numbers*.
//!
//! The values on the initial `seeds:` line come in pairs. Within each pair, the
//! first value is the *start* of the range and the second value is the *length*
//! of the range. So, in the first line of the example above:
//!
//! ```
//! seeds: 79 14 55 13
//! ```
//!
//! This line describes two ranges of seed numbers to be planted in the garden.
//! The first range starts with seed number `79` and contains `14` values: `79`,
//! `80`, ..., `91`, `92`. The second range starts with seed number `55` and
//! contains `13` values: `55`, `56`, ..., `66`, `67`.
//!
//! Now, rather than considering four seed numbers, you need to consider a total
//! of *27* seed numbers.
//!
//! In the above example, the lowest location number can be obtained from seed
//! number `82`, which corresponds to soil `84`, fertilizer `84`, water `84`,
//! light `77`, temperature `45`, humidity `46`, and *location `46`*. So, the
//! lowest location number is `*46*`.
//!
//! Consider all of the initial seed numbers listed in the ranges on the first
//! line of the almanac. *What is the lowest location number that corresponds to
//! any of the initial seed numbers?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](5/input).
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

#[derive(Debug)]
pub struct MapLine {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapLine {
    pub fn new(line: &str) -> Result<Self, Report> {
        let (destination_range_start, source_range_start, range_length) = line
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| eyre::eyre!("couldn't collect numbers"))?;
        Ok(MapLine {
            destination_range_start: destination_range_start.parse()?,
            source_range_start: source_range_start.parse()?,
            range_length: range_length.parse()?,
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
    pub fn translate(&self, from: usize) -> Option<usize> {
        if (self.source_range_start..self.source_range_start + self.range_length).contains(&from) {
            Some(self.destination_range_start + (from - self.source_range_start))
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
}

impl Solver<Year2023, Day5, Part1> for Solution {
    type Input<'a> = (Vec<usize>, Vec<Map<'a>>);

    type Output = usize;

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
                seed = map
                    .lines
                    .iter()
                    .find_map(|line| line.translate(seed))
                    .unwrap_or(seed);
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
            .flat_map(|[start, range]: [usize;2] | start..(start+range))
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
