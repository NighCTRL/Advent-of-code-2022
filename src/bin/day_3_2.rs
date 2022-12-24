#![feature(iter_array_chunks)]
use std::collections::{HashMap, HashSet};

use anyhow::{Result, Ok};

const START_LOWER: u8 = b'a' - 1;
const START_UPPER: u8 = b'A' - 1;

fn main () -> Result<()> {
    let foo = std::fs::read_to_string("src/inputs/day_3.input")?
        .lines()
        .array_chunks::<3>()
        .flat_map(|line| {
            return line 
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c | {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3)
        })
        .map(|c| c.0)
        .map(|c| { 
            let value = if c.is_ascii_lowercase() {
                c as u8 - START_LOWER
            } else {
            c as u8 - START_UPPER + 26
            } as u32;
            return value;
        })
        .sum::<u32>();

    println!("foo {}", foo);
    return Ok(());
}
