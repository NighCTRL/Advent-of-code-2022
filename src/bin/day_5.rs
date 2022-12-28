use std::str::{FromStr, Chars};
use anyhow::Result;

fn main () -> Result<()> {
    // maximum crates numbers in one stack on initial input
    let stack_layers = 9;

    // this will store all the stacks 
    let mut stacks: Vec<String> = vec!["".to_string()];

    // create as much stacks in the stacks array as needed
    let mut iterator = 1;
    while iterator < stack_layers {
        stacks.insert(iterator, "".to_string());
        iterator += 1;
    }

    // each line of the input file
    let lines: Vec<String>= std::fs::read_to_string("src/inputs/day_5.input")?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // map over each line of the input file and assign crates found on each lines to the correct
    // stack crate
    for (i, line) in lines.iter().enumerate() {

        if i >= stack_layers {
            break;
        }

        for (m, character) in line.chars().enumerate() {
            if character.is_uppercase() {
                match m {
                    1 => stacks[0] = stacks[0].to_owned() + &character.to_string(),
                    5 => stacks[1] = stacks[1].to_owned() + &character.to_string(),
                    9 => stacks[2] = stacks[2].to_owned() + &character.to_string(),
                    13 => stacks[3] = stacks[3].to_owned() + &character.to_string(),
                    17 => stacks[4] = stacks[4].to_owned() + &character.to_string(),
                    21 => stacks[5] = stacks[5].to_owned() + &character.to_string(),
                    25 => stacks[6] = stacks[6].to_owned() + &character.to_string(),
                    29 => stacks[7] = stacks[7].to_owned() + &character.to_string(),
                    33 => stacks[8] = stacks[8].to_owned() + &character.to_string(),
                    _ => (),
                }
            }
        }
    }
    // now execute the orders below the crates disposition, starting at line 11
    for (i, line) in lines.iter().enumerate() {
        if i >= 10 {
            // each line has 3 numbers representing the crates to move, from where, and the
            // destionation
            let mut orders: Vec<i8> = Vec::new();
            for token in line.split(" ") {
                match i8::from_str(token) {
                    Ok(n) => orders.push(n),
                    Err(_) => (),
                }
            }
            let crates_to_move = orders[0] as usize;
            let origin_stack_id = orders[1] as usize -1;
            let destination_stack_id = orders[2] as usize -1;
            println!("----------------------");
            println!("i need to move {} crates from stack[{}] to stack[{}]", crates_to_move, origin_stack_id, destination_stack_id);
            // we then move the crates accordingly, sometime we need to move more crates than what
            // the stack has, if that's the case we move only the available crates, and not the
            // more than needed, avoiding a crash.
            if crates_to_move > stacks[origin_stack_id].len() {
                println!("BIGGER crates to move are bigger than len of originstack");
                stacks[origin_stack_id] = stacks[origin_stack_id][..(crates_to_move -1)].to_string();
                let moved_crates = stacks[origin_stack_id][..(stacks[origin_stack_id].len())].to_string().chars().rev().collect::<String>();
                let stack_len = stacks[origin_stack_id].len();
                stacks[destination_stack_id] = format!("{}{}", moved_crates, stacks[destination_stack_id]);
                stacks[origin_stack_id] = stacks[origin_stack_id].chars().skip(stack_len).collect::<String>();
            } else {
                let moved_crates = stacks[origin_stack_id][..crates_to_move].to_string().chars().rev().collect::<String>();
                stacks[destination_stack_id] = format!("{}{}", moved_crates, stacks[destination_stack_id]);
                stacks[origin_stack_id] = stacks[origin_stack_id].chars().skip(orders[0].try_into().unwrap()).collect::<String>();
            }
            println!("stacks: {:?}", stacks);
        }
    }
    // we need to know the heighest crates on each stack, so we take the first character of each
    // stack
    let heighest_crates = stacks
        .into_iter()
        .map(|line| {
            line.chars().next().unwrap()
        })
        .collect::<Vec<_>>();
    println!("heighest of crates of the stacks: {:?}", heighest_crates);
    return Ok(());
}
