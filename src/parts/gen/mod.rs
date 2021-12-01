use std::{io::Write, path::PathBuf};

pub mod days;
pub mod years;

#[test]
#[ignore]
fn sourcegen_days() {
    let mut contents = String::from("");
    for day in 1..=25 {
        contents.push_str(&format!("pub const Day{day}: u32 = {day};\n"));
    }
    let mut file = std::fs::File::create(project_root().join("src/parts/gen/days.rs")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

#[test]
#[ignore]
fn sourcegen_years() {
    let mut contents = String::from("");
    for year in 2015..=2025 {
        contents.push_str(&format!("pub const Year{year}: u32 = {year};\n"));
    }
    let mut file = std::fs::File::create(project_root().join("src/parts/gen/years.rs")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

#[cfg(test)]
pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    dbg!(PathBuf::from(dir))
}
