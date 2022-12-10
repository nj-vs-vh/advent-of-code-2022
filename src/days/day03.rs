use crate::solution::Solution;
use crate::visualizer::Visualizer;
use itertools::Itertools;
use std::collections::{hash_map::RandomState, HashSet};

fn find_misplaced_item(rucksack: String) -> char {
    let compartment_size = rucksack.len() / 2;
    let first: HashSet<char> = rucksack[..compartment_size].chars().collect();
    let second: HashSet<char> = rucksack[compartment_size..].chars().collect();

    first.intersection(&second).next().unwrap().clone()
}

fn find_badge(group_rucksacks: &Vec<String>) -> char {
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

pub struct RucksacksReorganization;

impl Solution for RucksacksReorganization {
    type InputT = Vec<String>;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw.lines().map(|s| s.to_owned()).collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .into_iter()
            .map(find_misplaced_item)
            .map(item_priority)
            .sum()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .into_iter()
            .chunks(3)
            .into_iter()
            .map(|chunk| find_badge(&(chunk.collect())))
            .map(item_priority)
            .sum::<u32>()
    }
}
