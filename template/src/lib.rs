#![feature(generic_associated_types)]
//! Advent of code {{year}} {{day}}
//! 
//! https://adventofcode.com/{{year}}/day/{{day}}
//! 
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//! {{brief}}
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//! 
//! * 

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

    fn solve_first(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        todo!()
    }

    fn solve_second(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
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
    assert_eq!(Solution::solve_first(&input).unwrap(), 0);
}

#[test]
fn test_solution_second() {
    let input = r#"
0
    "#;
    let input = Solution::generate_input(input).unwrap();
    assert_eq!(Solution::solve_second(&input).unwrap(), 0);
}