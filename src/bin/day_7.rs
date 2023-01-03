use std::collections::HashMap;
use anyhow::{Ok, Result};
fn main () -> Result<()> {
    let mut cwd: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    std::fs::read_to_string("src/inputs/day_7.input")?
    .lines()
        .for_each(|line| {
            match line.chars().next() {
                Some('$') => handle_cmd(&mut cwd, &mut dir_sizes, line.to_string()),
                Some(_) => handle_file(&cwd, &mut dir_sizes, line.to_string()),
                None => (),
            }
        });
    let total: i32 = dir_sizes.values().filter(|&size| *size < 100000).sum();
    println!("total size of dir < 100000 = {}", total);
    println!("min size deletion to update = {}", what_to_delete(&dir_sizes));
    return Ok(());
}
fn handle_cmd (cwd: &mut Vec<String>, dir_sizes: &mut HashMap<String, i32>, line: String) {
    let command:Vec<&str> = line.split_whitespace().collect();
    if command[1] == "cd" {
        if command[2] == ".." && cwd.last().unwrap() != "root" {
            cwd.pop();
        } 
        else if command[2] == "/" {
            cwd.clear();
            cwd.push("root".to_string());
        } 
        else {
            cwd.push(command[2].to_string());
        }
        let path_string = cwd.join("/");
        if ! dir_sizes.contains_key(&path_string) {
            dir_sizes.insert(path_string, 0);
        };
    }
}
fn handle_file (cwd: &Vec<String>, dir_sizes: &mut HashMap<String, i32>, line: String) {
    let output: Vec<&str> = line.split_whitespace().collect();
    if output[0] != "dir" {
        let filesize: i32 = output[0].parse::<i32>().unwrap();
        let mut update_dir_list: Vec<String> = Vec::new();
        for dir in cwd {
            update_dir_list.push(dir.to_string());
            dir_sizes.entry(update_dir_list.join("/")).and_modify(|size| *size += filesize);
        }
    }
}
fn what_to_delete (dir_sizes: & HashMap<String, i32>) -> i32 {
    let minimum_delete = 30000000 - (70000000 - dir_sizes.get("root").unwrap());
    let mut delete_dir_size = 30000000;
    for (_, dir_size) in dir_sizes {
        if dir_size > &minimum_delete && dir_size < &delete_dir_size {
            delete_dir_size = *dir_size;
        }
    }
    return delete_dir_size;
}
