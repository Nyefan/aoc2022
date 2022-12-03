pub fn part1(filepath: &str) {
    let &result = read_input_to_vec(filepath)
        .iter()
        .max()
        .unwrap();

    println!("{}", result);
}

pub fn part2(filepath: &str) {
    let mut lines = read_input_to_vec(filepath);
    lines.sort_by_key(|&v| std::cmp::Reverse(v));
    let result = lines.iter()
        .take(3)
        .sum::<u64>();

    println!("{:?}", result);
}

fn read_input_to_vec(filepath: &str) -> Vec<u64> {
    std::fs::read_to_string(filepath)
        .expect(&format!("while reading {}", filepath))
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|group|
            group.split("\n")
                .map(|item| item.to_string().parse::<u64>().unwrap())
                .sum::<u64>())
        .collect::<Vec<u64>>()
}