//! Advent of code 2022 5
//!
//!! Link: <https://adventofcode.com/2022/day/5>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::str::FromStr;

use aoc::{parts::*, utils::*, Solver};
use eyre::Report;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Krate<'a>(pub &'a str);
#[derive(Clone, Debug)]
pub struct Stacks<'a> {
    pub arrays: Vec<Vec<Krate<'a>>>,
}

/// Indexes into a [Stacks::arrays] entry
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StackIndex {
    position: usize,
    stack_num: usize,
}

impl StackIndex {}

impl<'a> Stacks<'a> {
    pub fn new(stacks: Vec<Vec<Krate<'a>>>) -> Self {
        Self { arrays: stacks }
    }

    pub fn relocate(
        &mut self,
        amount: u32,
        from: usize,
        to: usize,
        multiple: bool,
    ) -> Result<(), eyre::Report> {
        let [from, to] = self.arrays.disjoint_mut([from, to])?;
        if multiple {
            to.extend(from.drain(from.len() - amount as usize..));
        } else {
            to.extend(from.drain(from.len() - amount as usize..).rev());
        };
        Ok(())
    }

    pub fn as_stacks(&self) -> Vec<Vec<Krate<'_>>> {
        todo!()
    }

    pub fn find_top(&self, stack: usize) -> Option<StackIndex> {
        return Some(StackIndex {
            position: self.arrays.get(stack)?.len() - 1,
            stack_num: stack,
        });
    }

    pub fn get_top(&self, stack: usize) -> Option<&Krate<'a>> {
        self.arrays.get(stack)?.last()
    }

    pub fn get(&self, idx: StackIndex) -> Option<&Krate<'a>> {
        let Some(stack) = self.arrays.get(idx.stack_num) else {
            return None;
        };
        stack.get(idx.position)
    }

    fn parse(stacks: &'a str) -> Result<Self, eyre::Report> {
        let mut arranged = vec![];
        let mut iter = stacks.lines().rev();
        let total: u32 = if let Some(sum) = iter.next() {
            let Some(total) = sum
                .trim()
                .rsplit_once(' ')
                .map(|s| s.1.parse())
                .transpose()?
            else {
                eyre::bail!("invalid input found")
            };
            total
        } else {
            eyre::bail!("oops")
        };
        arranged.extend(vec![vec![]; total as usize]);

        for line in iter {
            for (i, mut krate) in line.graphemes(true).chunks(4).into_iter().enumerate() {
                if let Some(krate) = krate.find(|s| !s.contains(['[', ']', ' '])) {
                    arranged.get_mut(i).unwrap().push(Krate(krate));
                };
            }
        }
        Ok(Self::new(arranged))
    }

    pub fn len(&self) -> usize {
        self.arrays.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Display for Stacks<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len();
        for row in self.arrays.iter() {
            writeln!(
                f,
                "{}",
                row.iter().map(|k| format!("[{}]", k.0)).format(" ")
            )?;
        }
        writeln!(f, "{len}")?;
        Ok(())
    }
}

#[test]
#[cfg(test)]
fn stack_mut() {
    let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
1   2   3
"#;
    let mut stacks = Stacks::parse(input).unwrap();
    let one = stacks.find_top(0).unwrap();
    assert_eq!(stacks.get(one).as_ref().unwrap().0, "N");
    let two = stacks.find_top(1).unwrap();
    assert_eq!(stacks.get(two).as_ref().unwrap().0, "D");
    println!("{stacks}");
    stacks.relocate(1, 1, 0, false).unwrap();
    println!("{stacks}");

    let one = stacks.find_top(0).unwrap();
    assert_eq!(stacks.get(one).as_ref().unwrap().0, "D");
    let two = stacks.find_top(1).unwrap();
    assert_eq!(stacks.get(two).as_ref().unwrap().0, "C");
    println!("{stacks}");
}

#[derive(thiserror::Error, Debug)]
pub enum InstructionParseError {
    #[error("instruction was invalid: `{rest}` @ {loc}")]
    Invalid {
        loc: &'static std::panic::Location<'static>,
        rest: String,
    },
    #[error("couldn't parse the number")]
    ParseNum(#[from] std::num::ParseIntError),
}

impl InstructionParseError {
    #[track_caller]
    pub fn invalid(s: &str) -> Self {
        Self::Invalid {
            loc: std::panic::Location::caller(),
            rest: s.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub amount: u32,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(rest) = s.strip_prefix("move ") else {
            return Err(InstructionParseError::invalid(s));
        };
        let Some((amount, rest)) = rest.split_once(' ') else {
            return Err(InstructionParseError::invalid(rest));
        };
        let Some(rest) = rest.strip_prefix("from ") else {
            return Err(InstructionParseError::invalid(rest));
        };
        let Some((from, rest)) = rest.split_once(' ') else {
            return Err(InstructionParseError::invalid(rest));
        };
        let Some(to) = rest.strip_prefix("to ") else {
            return Err(InstructionParseError::invalid(rest));
        };

        Ok(Instruction {
            amount: amount.parse()?,
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

impl Solver<Year2022, Day5, Part1> for Solution {
    type Input<'a> = (Stacks<'a>, Vec<Instruction>);

    type Output = String;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let Some((stacks, instructions)) = input.split_once("\n\n") else {
            eyre::bail!("invalid input")
        };
        // Arranges the crates from bottom up
        let instructions = instructions
            .trim()
            .lines()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok((Stacks::parse(stacks)?, instructions))
    }

    fn solve(
        (stacks, instructions): &(Stacks<'_>, Vec<Instruction>),
    ) -> Result<Self::Output, Report> {
        use std::fmt::Write;
        let mut stacks = stacks.clone();
        for instruction in instructions {
            for _ in 0..instruction.amount {
                stacks.relocate(1, instruction.from - 1, instruction.to - 1, false)?;
            }
            println!("{stacks}");
        }
        let mut res = String::new();
        for i in 0..stacks.len() {
            write!(
                &mut res,
                "{}",
                stacks.get_top(i).ok_or_else(|| eyre::eyre!("noo!"))?.0
            )?;
        }
        Ok(res)
    }
}

impl Solver<Year2022, Day5, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 5, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 5, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day5, Part1>>::generate_input(input)
    }

    fn solve(
        (stacks, instructions): &(Stacks<'_>, Vec<Instruction>),
    ) -> Result<Self::Output, Report> {
        use std::fmt::Write;
        let mut stacks = stacks.clone();
        for instruction in instructions {
            stacks.relocate(
                instruction.amount,
                instruction.from - 1,
                instruction.to - 1,
                true,
            )?;
            println!("{stacks}");
        }
        let mut res = String::new();
        for i in 0..stacks.len() {
            write!(
                &mut res,
                "{}",
                stacks.get_top(i).ok_or_else(|| eyre::eyre!("noo!"))?.0
            )?;
        }
        Ok(res)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day5, Part1>(input)?,
        "CMZ"
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day5, Part2>(input)?,
        "MCD"
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day5, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day5, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
