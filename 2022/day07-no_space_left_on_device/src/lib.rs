//! Advent of code 2022 7
//!
//!! Link: <https://adventofcode.com/2022/day/7>
//!
//! Good luck!
//!
//! ## Description
//! <!---STARTOFDESCRIPTION--->
//!You can hear birds chirping and raindrops hitting leaves as the expedition
//! proceeds. Occasionally, you can even hear much louder sounds in the
//! distance; how big do the animals get out here, anyway?
//!
//! The device the Elves gave you has problems with more than just its
//! communication system. You try to run a system update:
//!
//! ```
//! $ system-update --please --pretty-please-with-sugar-on-top
//! Error: No space left on device
//! ```
//!
//! Perhaps you can delete some files to make space for the update?
//!
//! You browse around the filesystem to assess the situation and save the
//! resulting terminal output (your puzzle input). For example:
//!
//! ```
//! $ cd /
//! $ ls
//! dir a
//! 14848514 b.txt
//! 8504156 c.dat
//! dir d
//! $ cd a
//! $ ls
//! dir e
//! 29116 f
//! 2557 g
//! 62596 h.lst
//! $ cd e
//! $ ls
//! 584 i
//! $ cd ..
//! $ cd ..
//! $ cd d
//! $ ls
//! 4060174 j
//! 8033020 d.log
//! 5626152 d.ext
//! 7214296 k
//! ```
//!
//! The filesystem consists of a tree of files (plain data) and directories
//! (which can contain other directories or files). The outermost directory is
//! called `/`. You can navigate around the filesystem, moving into or out of
//! directories and listing the contents of the directory you're currently in.
//!
//! Within the terminal output, lines that begin with `$` are *commands you
//! executed*, very much like some modern computers:
//!
//! * `cd` means *change directory*. This changes which directory is the current
//!   directory, but the specific result depends on the argument:
//!   * `cd x` moves *in* one level: it looks in the current directory for the
//!     directory named `x` and makes it the current directory.
//!   * `cd ..` moves *out* one level: it finds the directory that contains the
//!     current directory, then makes that directory the current directory.
//!   * `cd /` switches the current directory to the outermost directory, `/`.
//!
//! * `ls` means *list*. It prints out all of the files and directories
//!   immediately contained by the current directory:
//!   * `123 abc` means that the current directory contains a file named `abc`
//!     with size `123`.
//!   * `dir xyz` means that the current directory contains a directory named
//!     `xyz`.
//!
//! Given the commands and output in the example above, you can determine that
//! the filesystem looks visually like this:
//!
//! ```
//! - / (dir)
//!   - a (dir)
//!     - e (dir)
//!       - i (file, size=584)
//!     - f (file, size=29116)
//!     - g (file, size=2557)
//!     - h.lst (file, size=62596)
//!   - b.txt (file, size=14848514)
//!   - c.dat (file, size=8504156)
//!   - d (dir)
//!     - j (file, size=4060174)
//!     - d.log (file, size=8033020)
//!     - d.ext (file, size=5626152)
//!     - k (file, size=7214296)
//! ```
//!
//! Here, there are four directories: `/` (the outermost directory), `a` and `d`
//! (which are in `/`), and `e` (which is in `a`). These directories also
//! contain files of various sizes.
//!
//! Since the disk is full, your first step should probably be to find
//! directories that are good candidates for deletion. To do this, you need to
//! determine the *total size* of each directory. The total size of a directory
//! is the sum of the sizes of the files it contains, directly or indirectly.
//! (Directories themselves do not count as having any intrinsic size.)
//!
//! The total sizes of the directories above can be found as follows:
//!
//! * The total size of directory `e` is *584* because it contains a single file
//!   `i` of size 584 and no other directories.
//! * The directory `a` has total size *94853* because it contains files `f`
//!   (size 29116), `g` (size 2557), and `h.lst` (size 62596), plus file `i`
//!   indirectly (`a` contains `e` which contains `i`).
//! * Directory `d` has total size *24933642*.
//! * As the outermost directory, `/` contains every file. Its total size is
//!   *48381165*, the sum of the size of every file.
//!
//! To begin, find all of the directories with a total size of *at most 100000*,
//! then calculate the sum of their total sizes. In the example above, these
//! directories are `a` and `e`; the sum of their total sizes is `*95437*`
//! (94853 + 584). (As in this example, this process can count files more than
//! once!)
//!
//! Find all of the directories with a total size of at most 100000. *What is
//! the sum of the total sizes of those directories?*
//!
//! Your puzzle answer was `1206825`.
//!
//! The first half of this puzzle is complete! It provides one gold star: \*
//!
//! \--- Part Two ---
//! ----------
//!
//! Now, you're ready to choose a directory to delete.
//!
//! The total disk space available to the filesystem is `*70000000*`. To run the
//! update, you need unused space of at least `*30000000*`. You need to find a
//! directory you can delete that will *free up enough space* to run the update.
//!
//! In the example above, the total size of the outermost directory (and thus
//! the total amount of used space) is `48381165`; this means that the size of
//! the *unused* space must currently be `21618835`, which isn't quite the
//! `30000000` required by the update. Therefore, the update still requires a
//! directory with total size of at least `8381165` to be deleted before it can
//! run.
//!
//! To achieve this, you have the following options:
//!
//! * Delete directory `e`, which would increase unused space by `584`.
//! * Delete directory `a`, which would increase unused space by `94853`.
//! * Delete directory `d`, which would increase unused space by `24933642`.
//! * Delete directory `/`, which would increase unused space by `48381165`.
//!
//! Directories `e` and `a` are both too small; deleting them would not free up
//! enough space. However, directories `d` and `/` are both big enough! Between
//! these, choose the *smallest*: `d`, increasing unused space by `*24933642*`.
//!
//! Find the smallest directory that, if deleted, would free up enough space on
//! the filesystem to run the update. *What is the total size of that
//! directory?*
//!
//! Answer:
//!
//! Although it hasn't changed, you can still [get your puzzle input](7/input).
//! <!---ENDOFDESCRIPTION--->
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
