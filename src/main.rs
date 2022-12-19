fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    day1::part1("./data/day1/actual.txt")?;
    day1::part2("./data/day1/actual.txt")?;

    Ok(())
}
