use crate::utils::read_input;
use itertools::Itertools;
use std::collections::{hash_map::RandomState, HashSet};

fn find_misplaced_item(rucksack: &str) -> char {
    let compartment_size = rucksack.len() / 2;
    let first: HashSet<char> = rucksack[..compartment_size].chars().collect();
    let second: HashSet<char> = rucksack[compartment_size..].chars().collect();

    first.intersection(&second).next().unwrap().clone()
}

fn find_badge(group_rucksacks: &Vec<&str>) -> char {
    group_rucksacks
        .iter()
        .map(|r| HashSet::from_iter(r.chars()))
        .reduce(|a, b| {
            a.intersection(&b)
                .map(|c| c.to_owned())
                .collect::<HashSet<char, RandomState>>()
        })
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .to_owned()
}

fn item_priority(item: char) -> u32 {
    let offset = if item.is_ascii_lowercase() {
        96
    } else if item.is_ascii_uppercase() {
        64 - 26
    } else {
        panic!("can't calculate priority!")
    };
    item.to_owned() as u32 - offset
}

pub fn rucksacks_reorganization() {
    let input = read_input(3, false);

    println!(
        "pt1: {:?}",
        input
            .lines()
            .map(find_misplaced_item)
            .map(item_priority)
            .sum::<u32>()
    );

    println!(
        "pt2: {:?}",
        input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|chunk| find_badge(&(chunk.collect())))
            .map(item_priority)
            .sum::<u32>()
    )
}
