#![feature(generic_associated_types)]
//! Advent of code {{year}} {{day}}
//! 
//! https://adventofcode.com/{{year}}/day/{{day}}
//! 
//! ## Description
//! 
//! {{brief}}

use eyre::Report;
use aoc::Aoc;
use aoc::Solver;
use itertools::Itertools;

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
    color_eyre::install()?;
    let aoc = Aoc::new(&{{year}}, &{{day}})?;
    println!("⭐Solution found⭐ ::\n{}", aoc.solve::<Solution>()?);
    Ok(())
}

