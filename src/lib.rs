#![feature(generic_associated_types)]
pub mod aoc;

pub use aoc::{Aoc, Level};
use eyre::Report;

pub trait Solver {
    type Input<'a> where Self: 'a;
    type Output: std::fmt::Display;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report>;

    fn solve(input: Self::Input<'_>) -> Result<Self::Output, Report>;
}