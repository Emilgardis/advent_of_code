#![feature(generic_associated_types)]
pub mod aoc;
pub mod parts;
pub mod test_util;

pub use aoc::{Aoc, Level};
use eyre::Report;

pub trait Solver<const YEAR: u32, const DAY: u32, const PART: u32> {
    type Input<'a>
    where
        Self: 'a;
    type Output: std::fmt::Display;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report>;

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report>;
}

pub fn solve_with_input<
    S: Solver<YEAR, DAY, PART>,
    const YEAR: u32,
    const DAY: u32,
    const PART: u32,
>(
    input: &str,
) -> Result<S::Output, Report> {
    let input = S::generate_input(input)?;
    S::solve(&input)
}
