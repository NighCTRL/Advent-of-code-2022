use anyhow::Result;
fn main () -> Result<()> {
    let doublework: i32 = std::fs::read_to_string("src/inputs/day_4.input")?
        .lines()
        .map(|line| {
            // collect each integer of the line into vector
            let task_ids: Vec<i32> = line.split(|c: char|
                !c.is_digit(10)).filter_map(|y| y.parse().ok()).collect();

        // if task id range for one elf is contained into another that means work in double
        if task_ids[0] <= task_ids[2] && task_ids[1] >= task_ids[3] ||
            task_ids[0] >= task_ids[2] && task_ids[1] <= task_ids[3] {
            return 1;
        } else {
            return 0;
            }
        })
        .sum();

    println!("work in double = {}", doublework);

    return Ok(());
}
