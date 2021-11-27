use std::{fmt, collections::BTreeMap, path::{PathBuf, Path}};

use eyre::WrapErr;
use serde::{Serialize, Deserialize};

use crate::Solver;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    First,
    Second,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.serialize(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aoc {
    pub year: u32,
    pub day: u32,
    pub level: Level,
    pub title: String,
    pub stars: Option<u8>,
    pub solution: BTreeMap<Level, String>,
    pub input: String,
    brief: BTreeMap<Level, String>,
}

impl Aoc {
    pub fn new(year: &impl fmt::Display, day: &impl fmt::Display) -> eyre::Result<Self> {
        let root = find_root().wrap_err("when trying to find root_dir of aocf")?;
        Self::on_root_dir(&root, year, day)
    }
    pub fn on_root_dir(root: &Path, year: &impl fmt::Display, day: &impl fmt::Display) -> eyre::Result<Self> {
        let file_path = root.join(format!(".aocf/cache/aoc{year}_{day:0>2}.json"));
        let file =  std::fs::File::open(&file_path).wrap_err(format!("could not open aoc file on {file_path:?}"))?;
        serde_json::from_reader(&file).wrap_err("when trying to deserialize aoc").map_err(Into::into)
    }
    pub fn solve<S: Solver>(&self) -> eyre::Result<S::Output> {
        let input = S::generate_input(&self.input).wrap_err("could not create input")?;
        S::solve(input).wrap_err("could not solve")
    }
}

/// Find configuration directory in current directory or its ancestors
pub fn find_root() -> eyre::Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;

    let conf_dir = cwd.ancestors()
        .find(|dir| dir.join(".aocf").is_dir())
        .filter(|dir| dir.join(".aocf/config").is_file());

    match conf_dir {
        Some(dir) => Ok(dir.to_path_buf()),
        None => eyre::bail!("no configuration found, maybe you need to run `aocf init`"),
    }
}