use std::collections::HashMap;

use crate::solution::Solution;

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

fn parse_dir_sizes(terminal_output: &str) -> HashMap<Vec<&str>, u32> {
    let mut result: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current_dir: Vec<&str> = Vec::new();
    let mut current_dir_size: u32 = 0;

    for terminal_line in terminal_output.lines() {
        match TerminalCommand::parse(terminal_line) {
            Some(TerminalCommand::CD(dir)) => {
                let mut partial_dir: Vec<&str> = Vec::new();
                for current_dir_part in current_dir.iter() {
                    partial_dir.push(current_dir_part);
                    if !result.contains_key(&partial_dir) {
                        result.insert(partial_dir.clone(), 0);
                    }
                    *result.get_mut(&partial_dir).unwrap() += current_dir_size
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
        if !result.contains_key(&partial_dir) {
            result.insert(partial_dir.clone(), 0);
        }
        *result.get_mut(&partial_dir).unwrap() += current_dir_size
    }
    result
}

pub struct NoSpaceLeftOnDevice;

impl Solution for NoSpaceLeftOnDevice {
    type InputT = String;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
    }

    fn solve_pt1(&self, input: Self::InputT) -> Self::OutputT {
        parse_dir_sizes(&input)
            .values()
            .filter(|v| **v < 100_000)
            .sum()
    }

    fn solve_pt2(&self, input: Self::InputT) -> Self::OutputT {
        let dir_sizes = parse_dir_sizes(&input);
        let current_fs_size = dir_sizes[&vec!["/"]];
        let current_free_space = 70_000_000 - current_fs_size;
        let space_to_clean = 30_000_000 - current_free_space;
        *dir_sizes
            .values()
            .filter(|v| **v > space_to_clean)
            .min()
            .unwrap()
    }
}
