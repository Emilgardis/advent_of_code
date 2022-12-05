#![feature(iter_next_chunk)]
//! Advent of code 2022 5
//!
//! https://adventofcode.com/2022/day/5
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//! The expedition can depart as soon as the final supplies have been unloaded from the ships.
//! Supplies are stored in stacks of marked *crates*, but because the needed supplies are buried under many other crates,
//! the crates need to be rearranged.
//!
//! The ship has a *giant cargo crane* capable of moving crates between stacks.
//! To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps.
//! After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her *which* crate will end up where,
//! and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates *and* the rearrangement procedure (your puzzle input). For example:
//!
//! ```
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//!
//! ```
//!
//! In this example, there are three stacks of crates.
//! Stack 1 contains two crates:
//! crate `Z` is on the bottom, and
//! crate `N` is on top.
//! Stack 2 contains three crates; from bottom to top, they are crates `M`, `C`, and `D`.
//! Finally, stack 3 contains a single crate, `P`.
//!
//! Then, the rearrangement procedure is given.
//! In each step of the procedure, a quantity of crates is moved from one stack to a different stack.
//! In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! ```
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved *one at a time*, so the first crate to be moved (`D`) ends up below the second and third crates:
//!
//! ```
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//!
//! ```
//!
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved *one at a time*, crate `C` ends up below crate `M`:
//!
//! ```
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//!
//! ```
//!
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//!
//! ```
//!
//! The Elves just need to know *which crate will end up on top of each stack*; in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack 3, so you should combine these together and give the Elves the message `*CMZ*`.
//!
//! *After the rearrangement procedure completes, what crate ends up on top of each stack?*
//!
//! Your puzzle answer was `QNNTGTPFN`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a *CrateMover 9001*.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and *the ability to pick up and move multiple crates at once*.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//! ```
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! ```
//!
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! ```
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! ```
//!
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates *stay in the same order*, resulting in this new configuration:
//!
//! ```
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//!
//! ```
//!
//! Next, as both crates are moved from stack 2 to stack 1, they *retain their order* as well:
//!
//! ```
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//!
//! ```
//!
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate `C` that gets moved:
//!
//! ```
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//!
//! ```
//!
//! In this example, the CrateMover 9001 has put the crates in a totally different order: `*MCD*`.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. *After the rearrangement procedure completes, what crate ends up on top of each stack?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](5/input).
//! <!---ENDOFDESCRIPTION--->
//! ## Notes
//!
//! *

use std::str::FromStr;

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Krate<'a>(pub &'a str);
#[derive(Clone, Debug)]
pub struct Stacks<'a> {
    pub arrays: Vec<Vec<Option<Krate<'a>>>>,
}

/// Indexes into a [Stacks::arrays] entry
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StackIndex {
    row: usize,
    column: usize,
}

impl StackIndex {
    /// Get the crate above this crate
    pub fn get_above(&self) -> Self {
        Self {
            row: self.row + 1,
            column: self.column,
        }
    }
    /// Get the crate above this crate
    pub fn get_below(&self) -> Option<Self> {
        Some(Self {
            row: self.row.checked_sub(1)?,
            column: self.column,
        })
    }
    pub fn get_under(&self, amount: u32) -> Option<Self> {
        Some(Self {
            row: self.row.checked_sub(amount as usize)?,
            column: self.column,
        })
    }
    pub fn get_over(&self, amount: u32) -> Self {
        Self {
            row: self.row + amount as usize,
            column: self.column,
        }
    }
}

impl<'a> Stacks<'a> {
    pub fn new(stacks: Vec<Vec<Option<Krate<'a>>>>) -> Self {
        Self { arrays: stacks }
    }

    pub fn relocate_one(&mut self, from: usize, to: usize) -> Result<(), eyre::Report> {
        let from = self
            .find_top(from)
            .ok_or_else(|| eyre::eyre!("no crate found in {from}"))?;
        let to = self
            .find_top(to)
            .map(|s| s.get_above())
            .unwrap_or_else(|| StackIndex { row: 0, column: to });
        assert!(self.get(from).is_some());
        assert!(self.get(to).is_none());

        let [from, to] = self.get_mut([from, to])?;
        std::mem::swap(from.1, to.1);
        Ok(())
    }

    pub fn relocate_multiple(
        &mut self,
        amount: u32,
        from: usize,
        to: usize,
    ) -> Result<(), eyre::Report> {
        let from = self
            .find_top(from)
            .ok_or_else(|| eyre::eyre!("no crate found in {from}"))?;
        let mut indicies = Vec::from_iter((0..amount).filter_map(|i| from.get_under(i)));
        let amount = indicies.len();

        let to = self
            .find_top(to)
            .map(|s| s.get_above())
            .unwrap_or_else(|| StackIndex { row: 0, column: to });
        indicies.extend((0..amount).map(|i| to.get_over(i as u32)).rev());

        assert!(indicies[..amount]
            .iter()
            .map(|from| self.get(*from))
            .all(|k| k.is_some()));

        assert!(indicies[amount..]
            .iter()
            .map(|to| self.get(*to))
            .all(|k| k.is_none()));

        let mut krates = self.get_mut_dumb(&indicies)?;
        let (a, b) = krates.split_at_mut(amount);
        for (from, to) in a.iter_mut().zip(b.iter_mut()) {
            std::mem::swap(from.1, to.1);
        }
        Ok(())
    }

    pub fn as_stacks(&self) -> Vec<Vec<Krate<'_>>> {
        todo!()
    }

    pub fn find_top(&self, stack: usize) -> Option<StackIndex> {
        for (i, row) in self.arrays.iter().enumerate().rev() {
            let Some(Some(_)) = row.get(stack) else {
                continue;
            };
            return Some(StackIndex {
                row: i,
                column: stack,
            });
        }
        None
    }

    pub fn get_top(&self, stack: usize) -> Option<&Krate<'a>> {
        for (_i, row) in self.arrays.iter().enumerate().rev() {
            let Some(Some(krate)) = row.get(stack) else {
                continue;
            };
            return Some(krate);
        }
        None
    }

    pub fn get(&self, idx: StackIndex) -> &Option<Krate<'a>> {
        let Some(row) = self.arrays.get(idx.row) else {
            return &None
        };
        match row.get(idx.column) {
            Some(opt) => opt,
            None => &None,
        }
    }

    pub fn get_mut_dumb(
        &mut self,
        indices: impl AsRef<[StackIndex]>,
    ) -> Result<Vec<(StackIndex, &mut Option<Krate<'a>>)>, eyre::Report> {
        let indices = indices.as_ref();
        if !indices.iter().all_unique() {
            eyre::bail!("entries must be unique, found duplicate")
        }
        for idx in indices {
            self.make_available(*idx)?;
        }

        let mut arr: Vec<_> = self
            .arrays
            .iter_mut()
            .enumerate()
            .flat_map(|(row, column)| {
                column
                    .iter_mut()
                    .enumerate()
                    .filter_map(move |(column, krate)| {
                        if indices.iter().any(|i| i.row == row && i.column == column) {
                            Some((StackIndex { row, column }, krate))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        #[allow(clippy::needless_range_loop)]
        for idx in 0..indices.len() {
            let Some(pos) = arr.iter().position(|(i, _)| *i == indices[idx]) else {
                eyre::bail!("oops")
            };
            if pos != idx {
                let (pos, idx) = if pos > idx { (idx, pos) } else { (pos, idx) };
                let (a, b) = arr.split_at_mut(idx);

                std::mem::swap(&mut a[pos], &mut b[0]);
            }
        }

        Ok(arr)
    }

    pub fn get_mut<const N: usize>(
        &mut self,
        indices: [StackIndex; N],
    ) -> Result<[(StackIndex, &mut Option<Krate<'a>>); N], eyre::Report> {
        let indices = indices.as_ref();
        if !indices.iter().all_unique() {
            eyre::bail!("entries must be unique, found duplicate")
        }
        for idx in indices {
            self.make_available(*idx)?;
        }

        let mut arr: [_; N] = self
            .arrays
            .iter_mut()
            .enumerate()
            .flat_map(|(row, column)| {
                column
                    .iter_mut()
                    .enumerate()
                    .filter_map(move |(column, krate)| {
                        if indices.iter().any(|i| i.row == row && i.column == column) {
                            Some((StackIndex { row, column }, krate))
                        } else {
                            None
                        }
                    })
            })
            .next_chunk()
            .map_err(|_| eyre::eyre!("not enough data, this is a bug"))?;

        #[allow(clippy::needless_range_loop)]
        for idx in 0..N {
            let Some(pos) = arr.iter().position(|(i, _)| *i == indices[idx]) else {
                eyre::bail!("oops")
            };
            if pos != idx {
                let (pos, idx) = if pos > idx { (idx, pos) } else { (pos, idx) };
                let (a, b) = arr.split_at_mut(idx);

                std::mem::swap(&mut a[pos], &mut b[0]);
            }
        }

        Ok(arr)
    }

    pub fn make_available(
        &mut self,
        StackIndex { row, column }: StackIndex,
    ) -> Result<(), eyre::Report> {
        if row >= self.arrays.len() {
            self.arrays
                .extend((self.arrays.len()..=row).map(|_| vec![<_>::default(); column + 1]));
        }

        let Some(row_vec) = self.arrays.get_mut(row) else {
            eyre::bail!("oops, len = {}", self.arrays.len())
        };

        if column >= row_vec.len() {
            row_vec.extend((row_vec.len()..=column).map(|_| <_>::default()));
        }
        Ok(())
    }

    fn parse(stacks: &'a str) -> Self {
        let mut arranged = vec![];
        let mut row = vec![];
        for line in stacks.lines().rev().skip(1) {
            row.clear();
            for mut krate in line.graphemes(true).chunks(4).into_iter() {
                match krate.find(|s| !s.contains(['[', ']', ' '])) {
                    Some(krate) => {
                        row.push(Some(Krate(krate)));
                    }
                    _ => row.push(None),
                };
            }
            arranged.push(row.clone())
        }
        Self::new(arranged)
    }

    pub fn len(&self) -> usize {
        self.arrays.first().unwrap().len()
    }
}

impl std::fmt::Display for Stacks<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len();
        let empty = "   ";
        for row in self
            .arrays
            .iter()
            .rev()
            .skip_while(|v| v.iter().all(|k| k.is_none()))
        {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|k| k.as_ref().map(|k| format!("[{}]", k.0)))
                    .map(|s| s.unwrap_or(empty.to_owned()))
                    .format(" ")
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
    let mut stacks = Stacks::parse(input);
    let one = stacks.find_top(0).unwrap();
    assert_eq!(stacks.get(one).as_ref().unwrap().0, "N");
    let two = stacks.find_top(1).unwrap();
    assert_eq!(stacks.get(two).as_ref().unwrap().0, "D");
    println!("{stacks}");
    stacks.relocate_one(1, 0).unwrap();
    let one = stacks.find_top(0).unwrap();
    assert_eq!(stacks.get(one).as_ref().unwrap().0, "D");
    let two = stacks.find_top(1).unwrap();
    assert_eq!(stacks.get(two).as_ref().unwrap().0, "C");
    println!("{stacks}");
}

#[test]
#[cfg(test)]
fn stack_available() {
    let mut stacks = Stacks::new(vec![]);
    stacks
        .make_available(StackIndex {
            row: 10,
            column: 15,
        })
        .unwrap();
    assert_eq!(stacks.arrays.len(), 11);
    assert_eq!(stacks.arrays[9].len(), 16);
    stacks
        .make_available(StackIndex {
            row: 10,
            column: 15,
        })
        .unwrap();
    stacks
        .make_available(StackIndex {
            row: 17,
            column: 19,
        })
        .unwrap();
    stacks
        .make_available(StackIndex {
            row: 17,
            column: 20,
        })
        .unwrap();
    stacks
        .make_available(StackIndex {
            row: 15,
            column: 20,
        })
        .unwrap();
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
            return Err(InstructionParseError::invalid(s) );
        };
        let Some((amount, rest)) = rest.split_once(' ') else {
            return Err(InstructionParseError::invalid(rest) );
        };
        let Some(rest) = rest.strip_prefix("from ") else {
            return Err(InstructionParseError::invalid(rest) );
        };
        let Some((from, rest)) = rest.split_once(' ') else {
            return Err(InstructionParseError::invalid(rest) );
        };
        let Some(to) = rest.strip_prefix("to ") else {
            return Err(InstructionParseError::invalid(rest) );
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
        Stacks::parse(stacks);
        let instructions = instructions
            .trim()
            .lines()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        Ok((Stacks::parse(stacks), instructions))
    }

    fn solve(
        (stacks, instructions): &(Stacks<'_>, Vec<Instruction>),
    ) -> Result<Self::Output, Report> {
        use std::fmt::Write;
        let mut stacks = stacks.clone();
        for instruction in instructions {
            for _ in 0..instruction.amount {
                stacks.relocate_one(instruction.from - 1, instruction.to - 1)?;
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
            stacks.relocate_multiple(
                instruction.amount,
                instruction.from - 1,
                instruction.to - 1,
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
    let input = r#"
    [D]
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
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day5, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}
