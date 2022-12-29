use anyhow::{Ok, Result};

pub fn check_marker (position: usize, signal: &Vec<char>) -> bool {
    let mut pos_offset = 0;
    // main loop
    while pos_offset != 3 {
        let main_char = signal[position + pos_offset];
        let mut remaining_test = 3 - pos_offset;
        while remaining_test != 0 {
            if main_char == signal[position + pos_offset + remaining_test ] {
                return false;
            }
            remaining_test -= 1;
        }
        pos_offset += 1;
    }
    return true;
}

fn main () -> Result<()>{
    let signal: Vec<char> = std::fs::read_to_string("src/inputs/day_6.input")?
        .chars()
        .map(|c| {
            return c
        })
        .collect();

    let mut signal_found = false;
    let mut position = 0;

    while signal_found == false {
        if position >= signal.len() - 4 {
            println!("signal not found");
            break;
        }
        signal_found = check_marker(position, &signal);
        if signal_found {
            println!("signal is found at pos {}", position + 4);
        }
        position += 1;
    }

    return Ok(())
}
