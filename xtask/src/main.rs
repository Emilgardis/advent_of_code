use std::io::Write;

use eyre::{WrapErr, Result};

mod flags;

fn main() -> Result<()> {
    let flags = flags::App::from_env()?;

    match flags.subcommand {
        flags::AppCmd::NewDay(new_day) => {
            generate_day(&new_day).context("could not generate new day")?;
        }
    };

    Ok(())
}

fn generate_day(flags: &flags::NewDay) -> Result<()> {
    use inflections::case::to_snake_case;
    let (day, year);
    // First, checkout the day
    let (day, year) = if let (Some(day), Some(year)) = (&flags.day, &flags.year) {
        (day.as_str(), year.as_str())
    } else {
        let date = time::OffsetDateTime::now_utc();
        day = date.day().to_string();
        year = date.year().to_string();
        (day.as_str(), year.as_str())
    };
    xshell::cmd!("aocf checkout --day {day} --year {year}").run()?;
    xshell::cmd!("aocf fetch").run()?;

    // import the data.
    let root_dir = aoc::aoc::find_root()?;

    let data = aoc::Aoc::on_root_dir(&root_dir, &year, &day)?;
    // get the template files
    let template_dir = root_dir.join("template/.");

    let files = walkdir::WalkDir::new(&template_dir);

    let day_dir = dbg!(root_dir.join(format!("{year}/day{day:0>2}-{}", to_snake_case(&data.title))));

    // Now, generate the template
    for dir_entry in files {
        let dir_entry = dir_entry?;
        let path = dbg!(dir_entry.path());
        eprintln!("{:?}", path.display());
        if !path.is_file() {
            continue;
        }
        let contents = std::fs::read_to_string(path).context("could not real file {path:?}")?;
        let contents = contents.replace("{{year}}", year);
        let contents = contents.replace("{{day}}", day);
        let contents = contents.replace("{{title_snake}}", &to_snake_case(&data.title));
        let contents = contents.replace("{{title}}", &data.title);
        let contents = contents.replace("{{level}}", &data.level.to_string());
        
        let depth = dbg!(dir_entry.depth());
        let parent_len = path.components().count() - depth;
        let mut components = path.components();
        for _ in 0..parent_len {
            components.next();
        }
        let new_file = day_dir.join(components.as_path());
        if new_file.exists() && !flags.force {
            eyre::bail!("file already exists: {:?}", new_file);
        } else {
            std::fs::create_dir_all(new_file.parent().ok_or_else(|| eyre::eyre!("no parent for dir"))?).context("could not create parent dir")?;
        }

        std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(new_file)?.write_all(contents.as_bytes())?;
    }


    Ok(())
}
