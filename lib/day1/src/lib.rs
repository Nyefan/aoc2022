use std::str::FromStr;

use color_eyre::eyre::Context;

#[derive(Debug)]
struct Packs(Vec<Pack>);

#[derive(Debug)]
struct Pack(Vec<u64>);

impl Pack {
    fn total(&self) -> u64 {
        self.0.iter().sum()
    }
}

impl Packs {
    fn iter(&self) -> std::slice::Iter<Pack> {
        self.0.iter()
    }
}

impl std::fmt::Display for Packs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::str::FromStr for Packs {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packs = Packs(
            s.lines()
                .map(|v| v.parse::<u64>().ok())
                .collect::<Vec<_>>()
                .split(|line| line.is_none())
                .map(|group| Pack(group.iter().map(|line| line.unwrap()).collect::<Vec<_>>()))
                .collect::<Vec<_>>(),
        );
        Ok(packs)
    }
}

fn read_input_to_packs(filepath: &str) -> color_eyre::Result<Packs> {
    let input = std::fs::read_to_string(filepath).wrap_err(format!("reading {}", filepath))?;
    let packs = Packs::from_str(&input)?;
    Ok(packs)
}

pub fn part1(filepath: &str) -> color_eyre::Result<()> {
    let result = read_input_to_packs(filepath)?
        .iter()
        .map(Pack::total)
        .max()
        .unwrap();

    println!("{}", result);
    Ok(())
}

pub fn part2(filepath: &str) -> color_eyre::Result<()> {
    let mut lines = read_input_to_packs(filepath)?.0;
    lines.sort_by_key(|v| std::cmp::Reverse(v.total()));
    let result = lines.iter().take(3).map(Pack::total).sum::<u64>();

    println!("{:?}", result);
    Ok(())
}
