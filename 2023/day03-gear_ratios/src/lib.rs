//! Advent of code 2023 3
//!
//! Link: <https://adventofcode.com/2023/day/3>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use aoc::{parts::*, Solver};
use eyre::Report;
use itertools::Itertools;
#[derive(PartialEq, Eq)]

pub struct Schematic {
    board: String,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq)]
struct Part<'a> {
    number: usize,
    touches: Symbol<'a>,
}
#[derive(PartialEq, Eq)]

struct Symbol<'a> {
    kind: &'a str,
    idx: usize,
}

impl Schematic {
    fn get_2d_from_1d(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }
    pub fn new(input: &str) -> Self {
        let width = input.split_whitespace().next().unwrap().len();
        let board = input.split_whitespace().collect::<String>();
        let height = input.split_whitespace().count();

        Schematic {
            board,
            width,
            height,
        }
    }

    /// Check if range touches a symbol on the board
    ///
    /// # Examples
    ///
    /// ```rust
    /// use y2023_day3_gear_ratios::Schematic;
    /// let board = "123..
    ///              *....";
    /// let schematic = Schematic::new(board);
    /// assert!(schematic.touches_symbol(0..4), "under");
    ///
    /// let board = "123..
    ///              ...*.";
    /// let schematic = Schematic::new(board);
    /// assert!(schematic.touches_symbol(0..4), "diag");
    /// let board = "123..
    ///              ....*";
    /// let schematic = Schematic::new(board);
    /// assert!(!schematic.touches_symbol(0..4), "not touching diag");
    /// let board = "123.*
    ///              .....";
    /// let schematic = Schematic::new(board);
    /// assert!(!schematic.touches_symbol(0..4), "not touching right");
    /// let board = "123*.
    ///              .....";
    /// let schematic = Schematic::new(board);
    /// assert!(schematic.touches_symbol(0..4), "touching right");
    /// let board = "*23..
    ///              .....";
    /// let schematic = Schematic::new(board);
    /// assert!(schematic.touches_symbol(1..4), "touching left");
    ///
    /// let board = "*.3.*
    ///              .....";
    /// let schematic = Schematic::new(board);
    /// assert!(!schematic.touches_symbol(3..4), "not touching right or left");
    /// let board = "*...*
    ///              *.1.*
    ///              *...*";
    /// let schematic = Schematic::new(board);
    /// assert!(!schematic.touches_symbol(8..9), "not touching");
    /// ```
    pub fn touches_symbol(&self, range: std::ops::Range<usize>) -> Option<Symbol> {
        let (s, e) = (range.start, range.end);
        let (left, right) = {
            // If adding or subtracting 1 would go over to the next line, skip
            // adding or subtracing. going over the line, means a
            // new y coordinate, so we can grab the x,y, and check if y_(s-1)
            // and y_(e+1) != y_s

            let (_, y) = self.get_2d_from_1d(s);

            let y_sub = self.get_2d_from_1d(s.saturating_sub(1)).1;
            let y_add = self.get_2d_from_1d(e.saturating_add(1)).1;

            ((y_sub == y) as usize, (y_add == y) as usize)
        };
        // look left and right
        let left_s = &self.board[(s + left).saturating_sub(2)..(s + left).saturating_sub(1)];

        if !left_s.is_empty() && !left_s.contains(|c: char| c == '.' || c.is_ascii_digit()) {
            eprintln!("touching left: {}", &left_s);
            return Some(Symbol {
                kind: left_s,
                idx: (s + left).saturating_sub(2),
            });
        }
        let right_s = &self.board[e - 1 + right..e + right];
        if !right_s.contains(|c: char| c == '.' || c.is_ascii_digit()) {
            eprintln!("touching right {}", right_s);
            return Some(Symbol {
                kind: right_s,
                idx: e - 1 + right,
            });
        }
        // look up and down
        for i in s.saturating_sub(left)..e.saturating_add(right) {
            if let Some(up) = i.checked_sub(self.width) {
                if let Some(s) = self.board.get(up..up + 1) {
                    if !s.contains(|c: char| c == '.' || c.is_ascii_digit()) {
                        eprintln!("touching up: {s}");
                        return Some(Symbol { kind: s, idx: up });
                    }
                }
            };
            if let Some(down) = i.checked_add(self.width) {
                if let Some(s) = self.board.get(down..down + 1) {
                    if !s.contains(|c: char| c == '.' || c.is_ascii_digit()) {
                        eprintln!("touching down: {s}");
                        return Some(Symbol { kind: s, idx: down });
                    }
                }
            };
        }
        None
    }
    fn parts(&self) -> impl Iterator<Item = Part<'_>> + '_ {
        // split on dots
        self.board
            .split('.')
            .flat_map(|s| {
                // unclog
                s.split(|c: char| !c.is_ascii_digit())
            })
            .filter(|s| !s.is_empty())
            .filter_map(|word| {
                if !word.contains(|c: char| !c.is_ascii_digit()) {
                    // get the index of the substring
                    let start = self.board.as_ptr() as usize;
                    let range = word.as_bytes().as_ptr_range();
                    let (s_start, s_end) =
                        (range.start as usize - start, range.end as usize - start);

                    // sanity
                    debug_assert_eq!(word, &self.board[s_start..s_end]);

                    print!("checking: {word}, {}..{} -- ", s_start, s_end);
                    let res = self
                        .touches_symbol(s_start..s_end)
                        .map(|symbol: Symbol<'_>| Part {
                            number: word.parse::<usize>().unwrap(),
                            touches: symbol,
                        });
                    if res.is_some() {
                        println!("{word} is a part");
                    } else {
                        println!("{word} is not a part");
                    }
                    res
                } else {
                    println!("{word} is not a part");

                    None
                }
            })
    }
}

impl Solver<Year2023, Day3, Part1> for Solution {
    type Input<'a> = Schematic;

    type Output = usize;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        Ok(Schematic::new(input))
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        Ok(input.parts().map(|p| p.number).sum())
    }
}

impl Solver<Year2023, Day3, Part2> for Solution {
    type Input<'a> = <Self as Solver<2023, 3, Part1>>::Input<'a>;

    type Output = <Self as Solver<2023, 3, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2023, Day3, Part1>>::generate_input(input)
    }

    fn solve(input: &Self::Input<'_>) -> Result<Self::Output, Report> {
        let parts = input
            .parts()
            .filter(|p| p.touches.kind == "*")
            .collect_vec();
        // get all parts with same touch idx
        let dupes: std::collections::HashMap<_, Vec<_>> = parts
            .iter()
            .into_grouping_map_by(|p| p.touches.idx)
            .collect();
        let mut sum = 0;
        for parts in dupes.values() {
            if parts.len() > 2 {
                continue;
            }
            let Some(gear1) = parts.get(0) else { continue };
            let Some(gear2) = parts.get(1) else { continue };
            sum += gear1.number * gear2.number;
        }
        Ok(sum)
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
467..114..
...*......
..35./633.
..........
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day3, Part1>(input)?,
        4361
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2023, Day3, Part2>(input)?,
        467835
    );
    Ok(())
}

#[test]
fn print_input() {
    aoc::test_util::init();
    println!("{}", aoc::Aoc::new(&2023, &3).unwrap().input);
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day3, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2023, Day3, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
