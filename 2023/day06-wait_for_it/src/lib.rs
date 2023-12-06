//! Advent of code 2023 6
//!
//! Link: <https://adventofcode.com/2023/day/6>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!The ferry quickly brings you across Island Island. After asking around, you
//! discover that there is indeed normally a large pile of sand somewhere near
//! here, but you don't see anything besides lots of water and the small island
//! where the ferry has docked.
//!
//! As you try to figure out what to do next, you notice a poster on a wall near
//! the ferry dock. "Boat races! Open to the public! Grand prize is an
//! all-expenses-paid trip to *Desert Island*!" That must be where the sand
//! comes from! Best of all, the boat races are starting in just a few minutes.
//!
//! You manage to sign up as a competitor in the boat races just in time. The
//! organizer explains that it's not really a traditional race - instead, you
//! will get a fixed amount of time during which your boat has to travel as far
//! as it can, and you win if your boat goes the farthest.
//!
//! As part of signing up, you get a sheet of paper (your puzzle input) that
//! lists the *time* allowed for each race and also the best *distance* ever
//! recorded in that race. To guarantee you win the grand prize, you need to
//! make sure you *go farther in each race* than the current record holder.
//!
//! The organizer brings you over to the area where the boat races are held. The
//! boats are much smaller than you expected - they're actually *toy boats*,
//! each with a big button on top. Holding down the button *charges the boat*,
//! and releasing the button *allows the boat to move*. Boats move faster if
//! their button was held longer, but time spent holding the button counts
//! against the total race time. You can only hold the button at the start of
//! the race, and boats don't move until the button is released.
//!
//! For example:
//!
//! ```
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! This document describes three races:
//!
//! * The first race lasts 7 milliseconds. The record distance in this race is 9
//!   millimeters.
//! * The second race lasts 15 milliseconds. The record distance in this race is
//!   40 millimeters.
//! * The third race lasts 30 milliseconds. The record distance in this race is
//!   200 millimeters.
//!
//! Your toy boat has a starting speed of *zero millimeters per millisecond*.
//! For each whole millisecond you spend at the beginning of the race holding
//! down the button, the boat's speed increases by *one millimeter per
//! millisecond*.
//!
//! So, because the first race lasts 7 milliseconds, you only have a few
//! options:
//!
//! * Don't hold the button at all (that is, hold it for *`0` milliseconds*) at
//!   the start of the race. The boat won't move; it will have traveled *`0`
//!   millimeters* by the end of the race.
//! * Hold the button for *`1` millisecond* at the start of the race. Then, the
//!   boat will travel at a speed of `1` millimeter per millisecond for 6
//!   milliseconds, reaching a total distance traveled of *`6` millimeters*.
//! * Hold the button for *`2` milliseconds*, giving the boat a speed of `2`
//!   millimeters per millisecond. It will then get 5 milliseconds to move,
//!   reaching a total distance of *`10` millimeters*.
//! * Hold the button for *`3` milliseconds*. After its remaining 4 milliseconds
//!   of travel time, the boat will have gone *`12` millimeters*.
//! * Hold the button for *`4` milliseconds*. After its remaining 3 milliseconds
//!   of travel time, the boat will have gone *`12` millimeters*.
//! * Hold the button for *`5` milliseconds*, causing the boat to travel a total
//!   of *`10` millimeters*.
//! * Hold the button for *`6` milliseconds*, causing the boat to travel a total
//!   of *`6` millimeters*.
//! * Hold the button for *`7` milliseconds*. That's the entire duration of the
//!   race. You never let go of the button. The boat can't move until you let go
//!   of the button. Please make sure you let go of the button so the boat gets
//!   to move. *`0` millimeters*.
//!
//! Since the current record for this race is `9` millimeters, there are
//! actually `*4*` different ways you could win: you could hold the button for
//! `2`, `3`, `4`, or `5` milliseconds at the start of the race.
//!
//! In the second race, you could hold the button for at least `4` milliseconds
//! and at most `11` milliseconds and beat the record, a total of `*8*`
//! different ways to win.
//!
//! In the third race, you could hold the button for at least `11` milliseconds
//! and no more than `19` milliseconds and still beat the record, a total of
//! `*9*` ways you could win.
//!
//! To see how much margin of error you have, determine the *number of ways you
//! can beat the record* in each race; in this example, if you multiply these
//! values together, you get `*288*` (`4` \* `8` \* `9`).
//!
//! Determine the number of ways you could beat the record in each race. *What
//! do you get if you multiply these numbers together?*
//!
//! Your puzzle answer was `393120`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad [kerning](https://en.wikipedia.org/wiki/Kerning). There's really *only one race* - ignore the spaces between the numbers on each line.
//!
//! So, the example from before:
//!
//! ```
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! ...now instead means this:
//!
//! ```
//! Time:      71530
//! Distance:  940200
//! ```
//!
//! Now, you have to figure out how many ways there are to win this single race.
//! In this example, the race lasts for *`71530` milliseconds* and the record
//! distance you need to beat is *`940200` millimeters*. You could hold the
//! button anywhere from `14` to `71516` milliseconds and beat the record, a
//! total of `*71503*` ways!
//!
//! *How many ways can you beat the record in this one much longer race?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](6/input).
//! <!---ENDOFDESCRIPTION--->
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
        // -buttom_time^2 + allowed_time * button_time = record_distance
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
            let left = (a.powi(2) - 4.0 * r).sqrt()/2.0;
            let roots = [a / 2.0 - left + eps, a / 2.0 + left - eps];
            // this simplifies (ignoreing ceil and floor), can we use that fact?
            // floor(a/2 + left) - ceil(a/2 - left) + 1 =>
            // left*2 + 1
            // i'm not sure, the tricky part is that we need to offset by integer amounts also
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
