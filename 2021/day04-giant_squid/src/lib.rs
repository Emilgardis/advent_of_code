#![feature(generic_associated_types)]
//! Advent of code 2021 4
//!
//! https://adventofcode.com/2021/day/4
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;

pub const WIDTH: usize = 5;

#[derive(Debug, Clone)]
pub struct BingoGame {
    boards: Vec<Board>,
    sequence: Vec<u32>,
}

impl BingoGame {
    fn new(boards: Vec<Board>, sequence: Vec<u32>) -> BingoGame {
        Self { boards, sequence }
    }
    pub fn mark(&mut self, number: u32) {
        for (i, board) in &mut self.boards.iter_mut().enumerate() {
            board.mark(number);
            tracing::debug!("marked: {i}\n== == == == ==\n{board}\n== == == == ==\n");
        }
    }
    pub fn remove(&mut self, pos: usize) -> Board {
        self.boards.remove(pos)
    }
    pub fn boards(&self) -> &[Board] {
        &self.boards
    }

    pub fn bingo(&self) -> Option<(usize, &Board)> {
        self.boards.iter().find_position(|b| b.bingo())
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    /// 2d representation of the board
    numbers: Vec<u32>,
    /// numbers we've hit
    marked: Vec<bool>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use aoc::owo_colors::OwoColorize;
        for (i, (num, mark)) in self.numbers.iter().zip(self.marked.iter()).enumerate() {
            if *mark {
                write!(f, "{: >2} ", num.red())?;
            } else {
                write!(f, "{: >2} ", num)?;
            }
            if (i + 1) % WIDTH == 0 {
                writeln!(f)?;
            } else {
                write!(f, "")?;
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn new(numbers: Vec<u32>) -> Self {
        let marked = vec![false; numbers.len()];
        let s = Self {
            numbers: numbers.clone(),
            marked,
        };
        tracing::debug!("created board: {numbers:?}\n{s}");
        s
    }
    /// Determine if the board has a bingo
    pub fn bingo(&self) -> bool {
        for mut chunk in &self
            .iter_marked()
            .chain(self.iter_marked_transpose())
            .chunks(WIDTH)
        {
            let chunk = chunk.collect::<Vec<_>>();
            if chunk.iter().all(|&x| x) {
                return true;
            }
        }
        false
    }

    pub fn mark(&mut self, number: u32) {
        if let Some((pos, _)) = self.numbers.iter().find_position(|&&n| n == number) {
            self.marked[pos] = true;
        }
    }

    pub fn iter_marked(&self) -> BoardMarkerIter<'_> {
        BoardMarkerIter {
            board: &self.marked,
            index: (0, 0),
            transpose: false,
        }
    }
    pub fn iter_marked_transpose(&self) -> BoardMarkerIter<'_> {
        BoardMarkerIter {
            board: &self.marked,
            index: (0, 0),
            transpose: true,
        }
    }

    pub fn unmarked_sum(&self) -> usize {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.marked[*i])
            .fold(0, |acc, (_, n)| acc + *n as usize)
    }
}

#[derive(Debug)]
pub struct BoardMarkerIter<'a> {
    board: &'a [bool],
    index: (usize, usize),
    transpose: bool,
}

impl BoardMarkerIter<'_> {
    pub fn next_index(&mut self) {
        self.index.0 += 1;
        if self.index.0 == WIDTH {
            self.index.0 = 0;
            self.index.1 += 1;
        }
    }
}

impl Iterator for BoardMarkerIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let index = if !self.transpose {
            (self.index.1 * WIDTH) + self.index.0
        } else {
            (self.index.0 * WIDTH) + self.index.1
        };
        let r = self.board.get(index).copied();
        self.next_index();
        r
    }
}

impl Solver<Year2021, Day4, Part1> for Solution {
    type Input<'a> = BingoGame;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        let mut input = input.lines();
        let sequence = input
            .next()
            .ok_or_else(|| eyre::eyre!("no sequence found"))?
            .split(',')
            .map(|x| x.parse::<u32>())
            .try_collect()?;
        let mut boards = Vec::new();
        for board in &input.filter(|l| !l.trim().is_empty()).chunks(WIDTH) {
            let numbers = board
                .inspect(|l| tracing::debug!("current line {l:?}"))
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|n| n.parse().map_err::<eyre::Error, _>(Into::into))
                })
                .flatten()
                .try_collect()?;
            boards.push(Board::new(numbers));
        }
        Ok(BingoGame::new(boards, sequence))
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut game = input.clone();
        for seq in game.sequence.clone() {
            tracing::info!("sequence: {seq}");
            game.mark(seq);
            if let Some((pos, game)) = game.bingo() {
                tracing::info!("bingo: {pos}\n== == == == ==\n{game}\n== == == == ==");
                return Ok(game.unmarked_sum() * seq as usize);
            }
        }
        eyre::bail!("no bingo found")
    }
}

impl Solver<Year2021, Day4, Part2> for Solution {
    type Input<'a> = BingoGame;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2021, Day4, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let mut game = input.clone();
        let mut last_removed_board = Board::new(vec![100]);
        for seq in game.sequence.clone() {
            tracing::info!("sequence: {seq}");
            game.mark(seq);
            while let Some((pos, board)) = game.bingo() {
                tracing::info!("removing: {pos}\n== == == == ==\n{board}\n== == == == ==");
                // NLL FTW!!!
                last_removed_board = game.remove(pos).clone();
            }
            match last_removed_board {
                board if game.boards().is_empty() => {
                    tracing::info!("last board: \n== == == == ==\n{board}\n== == == == ==");

                    return Ok(board.unmarked_sum() * seq as usize);
                }
                _ => (),
            }
        }
        eyre::bail!("no bingo found")
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_vertical() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    7,4,9,5,11

    22  7 17 1  0
     8  4 23  4 24
    21  9 14 16  7
     6  5  3 18  5
     1 11 20 15 19
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day4, Part1>(input)?,
        2684
    );
    Ok(())
}

#[test]
fn test_horizontal() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    22,7,17,1,2

    22  7 17 1  2
     8  4 23  4 24
    21  9 14 16  7
     6  5  3 18  5
     1 11 20 15 19
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day4, Part1>(input)?,
        466
    );
    Ok(())
}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day4, Part1>(input)?,
        4512
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2021, Day4, Part2>(input)?,
        1924
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day4, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2021, Day4, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{}", s))
}
