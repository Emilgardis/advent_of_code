//! Advent of code 2022 7
//!
//!! Link: <https://adventofcode.com/2022/day/7>
//!
//! Good luck!
//!
//! ## Notes
//!
//! *

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use aoc::{parts::*, Solver};
use eyre::{Context, Report};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandOutput<'a> {
    pub command: Command<'a>,
    pub output: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command<'a> {
    Ls,
    Cd(Cd<'a>),
}

impl<'a> Command<'a> {
    fn parse(s: &'a str) -> Result<Self, eyre::Report> {
        if let Some(cd) = s.strip_prefix("cd ") {
            let cd = match cd.trim() {
                ".." => Cd::Back,
                "/" => Cd::Root,
                dir => Cd::Dir(dir),
            };
            Ok(Command::Cd(cd))
        } else if s == "ls" {
            Ok(Command::Ls)
        } else {
            eyre::bail!("invalid command found, {s:?}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cd<'a> {
    Back,
    Root,
    Dir(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entry<'a> {
    Dir(&'a str),
    File(u32, &'a str),
}

impl Solver<Year2022, Day7, Part1> for Solution {
    type Input<'a> = Vec<CommandOutput<'a>>;

    type Output = u64;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        input
            .split("$ ")
            .skip(1)
            .map(|s| {
                let Some((com, out)) = s.split_once('\n') else {
                    eyre::bail!("oops: {s:?}")
                };
                Ok(CommandOutput {
                    command: Command::parse(com)?,
                    output: out,
                })
            })
            .collect::<Result<_, _>>()
    }

    fn solve(input: &Vec<CommandOutput>) -> Result<Self::Output, Report> {
        let mut dirs = HashMap::<PathBuf, u64>::new();
        let mut cwd = PathBuf::from("/");
        for command in input {
            match &command.command {
                Command::Ls => {
                    for file in command.output.lines() {
                        let Some((a, b)) = file.split_once(' ') else {
                            eyre::bail!("invalid ls output")
                        };

                        match a {
                            "dir" => (),
                            // match dirs.entry(cwd.join(b)) {
                            // std::collections::hash_map::Entry::Occupied(_) => todo!(),
                            // std::collections::hash_map::Entry::Vacant(v) => {
                            //     v.insert(0);
                            // }
                            //  },
                            size => {
                                for dir in cwd.ancestors() {
                                    *dirs.entry(dir.to_owned()).or_default() +=
                                        size.parse::<u64>()?
                                }
                            }
                        }
                    }
                }
                Command::Cd(cd) => match cd {
                    Cd::Back => {
                        cwd.pop();
                    }
                    Cd::Root => cwd = PathBuf::from("/"),
                    Cd::Dir(dir) => cwd = cwd.join(dir),
                },
            }
        }
        Ok(dirs.values().filter(|&&size| size < 100000).copied().sum())
    }
}

impl Solver<Year2022, Day7, Part2> for Solution {
    type Input<'a> = <Self as Solver<2022, 7, Part1>>::Input<'a>;

    type Output = <Self as Solver<2022, 7, Part1>>::Output;

    fn generate_input(input: &'_ str) -> Result<Self::Input<'_>, Report> {
        <Self as Solver<Year2022, Day7, Part1>>::generate_input(input)
    }

    fn solve(input: &Vec<CommandOutput>) -> Result<Self::Output, Report> {
        let mut dirs = HashMap::<PathBuf, u64>::new();
        let mut cwd = PathBuf::from("/");
        for command in input {
            match &command.command {
                Command::Ls => {
                    for file in command.output.lines() {
                        let Some((a, b)) = file.split_once(' ') else {
                            eyre::bail!("invalid ls output")
                        };

                        match a {
                            "dir" => (),
                            // match dirs.entry(cwd.join(b)) {
                            // std::collections::hash_map::Entry::Occupied(_) => todo!(),
                            // std::collections::hash_map::Entry::Vacant(v) => {
                            //     v.insert(0);
                            // }
                            //  },
                            size => {
                                for dir in cwd.ancestors() {
                                    *dirs.entry(dir.to_owned()).or_default() +=
                                        size.parse::<u64>()?
                                }
                            }
                        }
                    }
                }
                Command::Cd(cd) => match cd {
                    Cd::Back => {
                        cwd.pop();
                    }
                    Cd::Root => cwd = PathBuf::from("/"),
                    Cd::Dir(dir) => cwd = cwd.join(dir),
                },
            }
        }
        let size = dirs
            .get(&PathBuf::from("/"))
            .ok_or_else(|| eyre::eyre!("no root??"))?;

        let rem = 70000000 - size;
        let toremove = 30000000 - rem;
        Ok(dirs
            .values()
            .filter(|&&s| s >= toremove)
            .copied()
            .min()
            .unwrap())
    }
}

pub struct Solution {}

impl Solution {}

#[test]
fn test_solution() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day7, Part1>(input)?,
        95437
    );
    Ok(())
}

#[test]
fn test_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    "#
    .trim();
    assert_eq!(
        aoc::solve_with_input::<Solution, Year2022, Day7, Part2>(input)?,
        24933642
    );
    Ok(())
}

#[test]
#[ignore]
fn solve_solution() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day7, Part1>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}

#[test]
#[ignore]
fn solve_solution_second() -> Result<(), Report> {
    aoc::test_util::init();
    aoc::Aoc::solve::<Solution, Year2022, Day7, Part2>()
        .map(|s| println!(":: ⭐Solution found⭐ ::\n{s}"))
}
