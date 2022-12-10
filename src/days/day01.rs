use crate::solution::Solution;

pub struct CalorieCouting;

impl Solution for CalorieCouting {
    type InputT = Vec<u32>;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
            .split("\n\n")
            .map(|s| s.split("\n").map(|s| s.parse::<u32>().unwrap()).sum())
            .collect()
    }

    fn solve_pt1(&self, input: Self::InputT) -> Self::OutputT {
        *(input.iter().max().unwrap())
    }

    fn solve_pt2(&self, mut input: Self::InputT) -> Self::OutputT {
        input.sort();
        input.reverse();
        input[0..3].iter().sum()
    }
}
