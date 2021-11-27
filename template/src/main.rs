
use y{{year}}_day{{day}}_{{title_snake}}::Solution;


pub fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    let aoc = aoc::Aoc::new(&{{year}}, &{{day}})?;
    println!("⭐Solution found⭐ ::\n{}", aoc.solve_first::<Solution>()?);
    Ok(())
}

