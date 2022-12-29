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

pub fn check_message (position: usize, signal: &Vec<char>) -> bool {
    let mut pos_offset = 0;
    // main loop
    while pos_offset != 13 {
        let main_char = signal[position + pos_offset];
        let mut remaining_test = 13 - pos_offset;
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
            println!("start-of-packet marker is complete at character {}", position + 4);
        }
        position += 1;
    }

    let mut message_found = false;
    
    while message_found == false {
        if position >= signal.len() - 14 {
            println!("message not found");
            break;
        }
        message_found = check_message(position, &signal);
        if message_found {
            println!("start-of-message marker is at character {}", position + 14);
        }
        position += 1;
    }

    return Ok(())
}
