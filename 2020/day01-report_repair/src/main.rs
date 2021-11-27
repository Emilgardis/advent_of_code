#![feature(generic_associated_types)]
//! Advent of code 2020 1
//! 
//! https://adventofcode.com/2020/day/1
//! 
//! ## Description
//! 
//! {{brief}}


use eyre::Report;
use aoc::Aoc;
use aoc::Solver;

impl Solver for Solution {
    type Input<'a> = ();

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        todo!()
    }

    fn solve(input: Self::Input<'_>) -> Result<Self::Output, Report> {
        todo!()
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() {
    let input = r#"
0
    "#;
    let input = Solution::generate_input(input).unwrap();
    assert_eq!(Solution::solve(input).unwrap(), 0);
}

pub fn main() -> Result<(), eyre::Report> {
    let aoc = Aoc::new(&2020, &1)?;
    aoc.solve::<Solution>()?;
    Ok(())
}