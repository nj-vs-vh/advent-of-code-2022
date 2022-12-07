use std::collections::HashMap;

use crate::utils::read_input;

const DAY: u8 = 7;

#[derive(Debug)]
enum TerminalCommand<'b> {
    LS,
    CD(&'b str),
}

impl TerminalCommand<'_> {
    fn parse(line: &str) -> Option<TerminalCommand> {
        let mut words_iter = line.split(" ");
        if words_iter.next().unwrap() != "$" {
            return None;
        }
        match words_iter.next().unwrap() {
            "ls" => return Some(TerminalCommand::LS),
            "cd" => return Some(TerminalCommand::CD(words_iter.next().unwrap())),
            _ => panic!("Unknown command"),
        }
    }
}

pub fn no_space_left_on_device() {
    let input = read_input(DAY, false);

    let mut dir_sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current_dir: Vec<&str> = Vec::new();
    let mut current_dir_size: u32 = 0;

    for terminal_line in input.lines() {
        match TerminalCommand::parse(terminal_line) {
            Some(TerminalCommand::CD(dir)) => {
                let mut partial_dir: Vec<&str> = Vec::new();
                for current_dir_part in current_dir.iter() {
                    partial_dir.push(current_dir_part);
                    if !dir_sizes.contains_key(&partial_dir) {
                        dir_sizes.insert(partial_dir.clone(), 0);
                    }
                    *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
                }
                current_dir_size = 0;
                if dir == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(dir);
                }
            }
            Some(TerminalCommand::LS) => {}
            None => {
                let mut words = terminal_line.split(" ");
                let maybe_file_size = words.next().unwrap();
                if maybe_file_size != "dir" {
                    let file_size = maybe_file_size
                        .parse::<u32>()
                        .expect("File size must be a number");
                    current_dir_size += file_size;
                }
            }
        }
    }

    // code repeated but whatever
    let mut partial_dir: Vec<&str> = Vec::new();
    for current_dir_part in current_dir.iter() {
        partial_dir.push(current_dir_part);
        if !dir_sizes.contains_key(&partial_dir) {
            dir_sizes.insert(partial_dir.clone(), 0);
        }
        *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
    }

    println!("{:?}", dir_sizes);

    let small_dir_size_sum: u32 = dir_sizes.values().filter(|v| **v < 100000).sum();
    println!("pt1: {}", small_dir_size_sum);

    let current_fs_size = dir_sizes[&vec!["/"]];
    let current_free_space = 70_000_000 - current_fs_size;
    let space_to_clean = 30_000_000 - current_free_space;
    let min_enough_dir_size: u32 = *dir_sizes
        .values()
        .filter(|v| **v > space_to_clean)
        .min()
        .unwrap();
    println!("pt2: {}", min_enough_dir_size);
}
