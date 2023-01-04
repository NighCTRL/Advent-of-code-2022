use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let forest: Vec<Vec<i8>> = std::fs::read_to_string("src/inputs/day_8.input")?
        .lines()
        .map(|row| {
            row.chars().map(|char| char.to_string().parse::<i8>().unwrap()).collect()
        }).collect();
    let mut biggest_visibility = 0;
    let mut y_axis = 1;
    while y_axis < forest.len() -1 {
        let mut x_axis = 1;
        while x_axis < forest[y_axis].len() - 1 {
            let tree = forest[y_axis].iter().nth(x_axis).unwrap();
            let visible_trees: Vec<usize> = vec![
                (forest[y_axis][0..x_axis]
                    .iter()
                    .rev()
                    .collect::<Vec<_>>().iter().position(|&i| i >= &tree )
                    .unwrap_or(forest[y_axis][0..x_axis - 1 ].len()) + 1), // num of trees
                (forest[y_axis][x_axis + 1 ..forest[y_axis].len()]
                    .iter()
                    .position(|&i| i >= *tree)
                    .unwrap_or(forest[y_axis][x_axis + 1..forest[y_axis].len()].len() - 1) + 1), // right
                (forest[0..y_axis].iter().map(|row| row[x_axis]).collect::<Vec<_>>().iter().rev()
                    .position(|&i| i >= *tree)
                    .unwrap_or(forest[0..y_axis].len() - 1) + 1), // up
                (forest[y_axis + 1..forest.len()].iter()
                    .map(|row| row[x_axis]).collect::<Vec<_>>().iter()
                    .position(|&i| i >= *tree)
                    .unwrap_or(forest[y_axis + 1..forest.len()].len() - 1) + 1), // down
            ];
            let visibility = visible_trees[2] * visible_trees[0] * visible_trees[1] * visible_trees[3];
            if visibility > biggest_visibility {
                biggest_visibility = visibility;
            }
            x_axis += 1;
        }
        y_axis += 1;
    }
    println!("biggest visibility = {}", biggest_visibility);
    return Ok(());
}
