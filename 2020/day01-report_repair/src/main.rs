use y2020_day1_report_repair::Solution;

pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    let aoc = aoc::Aoc::new(&2020, &1)?;
    println!("⭐Solution found⭐ ::\n{}", aoc.solve_first::<Solution>()?);
    Ok(())
}
