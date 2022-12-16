use anyhow::Result;

fn main() -> Result<()> {
    let mut max = include_str!("../inputs/day_1.input")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .flat_map(str::parse::<usize>)
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));
    
    println!("The top 3 elves carry {:?}", max
        .iter()
        .take(3)
        .sum::<usize>());

    return Ok(());
}
