const START_LOWER: u8 = b'a' - 1;
const START_UPPER: u8 = b'A' - 1;
use anyhow::Result;
use std::collections::HashSet;

fn main () -> Result<()> {
    let points = std::fs::read_to_string("src/inputs/day_3.input")?
        .lines()
        .flat_map(|line| {
            let (part1, part2) = line.split_at(line.len() / 2);
            let part1 = part1.chars().collect::<HashSet<_>>();
            return part2
                .chars()
                .filter(move |c| part1.contains(c))
                .collect::<HashSet::<char>>()
                .into_iter()
                .map(|c| {
                    let value = if c.is_ascii_lowercase() {
                        c as u8 - START_LOWER
                    } else {
                        c as u8 - START_UPPER + 26
                    };
                    println!("C: {} value: {}", c, value);
                    return value;
                })
                .map(|c| c as u32)
        })
        .sum::<u32>();
    println!("Result: {}", points);
    return Ok(());
}
