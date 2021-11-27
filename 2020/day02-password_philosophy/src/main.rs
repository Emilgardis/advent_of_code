
use y2020_day2_password_philosophy::Solution;


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    let aoc = aoc::Aoc::new(&2020, &2)?;
    println!("⭐Solution found⭐ ::\n{}", aoc.solve_first::<Solution>()?);
    Ok(())
}

