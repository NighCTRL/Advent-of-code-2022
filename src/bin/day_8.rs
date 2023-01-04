use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let forest: Vec<Vec<i8>> = std::fs::read_to_string("src/inputs/day_8.input")?
        .lines()
        .map(|row| {
            row.chars().map(|char| char.to_string().parse::<i8>().unwrap()).collect()
        }).collect();
    let mut visible_trees = 0;
    let mut y_axis = 1;
    while y_axis < forest.len() -1 {
        let mut x_axis = 1;
        while x_axis < forest[y_axis].len() - 1 {
            let tree = forest[y_axis].iter().nth(x_axis).unwrap();
            let heighest_surr_trees: Vec<i8> = vec![
                (*forest[y_axis][0..x_axis].iter().max().unwrap()), // heihghest tree to the left
                (*forest[y_axis][x_axis + 1..forest[y_axis].len()].iter().max().unwrap()), // right
                (forest[0..y_axis].iter().map(|row| row[x_axis]).max().unwrap()), // up
                (forest[y_axis + 1..forest.len()].iter().map(|row| row[x_axis]).max().unwrap()) // down
            ];
            let tree_visible = heighest_surr_trees.iter().any(|front| *front < *tree);
            if tree_visible { visible_trees += 1 };
            x_axis += 1;
        }
        y_axis += 1;
    }
    // add trees at the forest borders to the visibility count.
    visible_trees += (forest.len() -1) * 4;
    println!("tree_visible = {}", visible_trees);
    return Ok(());
}
