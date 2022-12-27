use anyhow::Result;

fn main () -> Result<()> {
    let doublework: i32 = std::fs::read_to_string("src/inputs/day_4.input")?
        .lines()
        .map(|line| {
            // collect each integer of the line into vector
            let task_ids: Vec<i32> = line.split(|c: char|
                !c.is_digit(10)).filter_map(|y| y.parse().ok()).collect();

            // if task begining id end or beginings are equals, work do overlap
            if task_ids[0] == task_ids[2] ||
            task_ids[0] == task_ids[3] ||
            task_ids[1] == task_ids[3] ||
            task_ids[1] == task_ids[2] ||
            // if task range 2 or 3 is contained in the range of task id 0 or 1
            task_ids[2] > task_ids[0] && task_ids[2] < task_ids[1] || 
            task_ids[3] < task_ids[1] && task_ids[3] > task_ids[0] || 
            // if task id range 0 or 1 is contained in the range of task id 2 or 3
            task_ids[0] > task_ids[2] && task_ids[0] < task_ids[3] || 
            task_ids[1] > task_ids[2] && task_ids[1] < task_ids[3] {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("work in double = {}", doublework);

    return Ok(());
}
