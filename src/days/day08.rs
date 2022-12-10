use crate::visualizer::Visualizer;
use std::fmt::Display;

use crate::solution::Solution;
#[allow(unused_imports)]
use crate::utils::{print_2d_vec, read_input};

#[derive(Debug)]
pub struct Forest {
    tree_heights: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

type VisibilityMap = Vec<Vec<bool>>;

#[allow(dead_code)]
fn print_visibility_mask(vm: &VisibilityMap) {
    for row in vm {
        for item in row {
            print!("{}", if *item { "x" } else { "." });
        }
        println!("");
    }
    println!("");
}

type ScenicScoreMap = Vec<Vec<u32>>;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    TB,
    BT,
    LR,
    RL,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        return *self == Direction::TB || *self == Direction::BT;
    }
}

impl Forest {
    fn parse(input: &str) -> Forest {
        let mut th: Vec<Vec<u8>> = Vec::new();
        for (i, line) in input.lines().enumerate() {
            th.push(Vec::new());
            for character in line.chars() {
                th[i].push(character.to_string().parse().unwrap())
            }
        }
        let width = th.len();
        let height = th[0].len();
        Forest {
            tree_heights: th,
            width: width,
            height: height,
        }
    }

    fn empty_visibility_map(&self) -> VisibilityMap {
        vec![vec![false; self.width]; self.height]
    }

    fn visibility_map(&self, direction: &Direction) -> VisibilityMap {
        let mut visibility_map: VisibilityMap = self.empty_visibility_map();

        let los_range = if direction.is_vertical() {
            0..self.width
        } else {
            0..self.height
        };

        let depth_range: Vec<usize> = match direction {
            Direction::TB => (0..self.height).collect(),
            Direction::BT => (0..self.height).rev().collect(),
            Direction::LR => (0..self.width).collect(),
            Direction::RL => ((0..self.width).rev()).collect(),
        };

        for los_idx in los_range {
            let mut max_height = 0;
            for (depth_iter_idx, depth_idx) in depth_range.iter().enumerate() {
                let (i, j) = if direction.is_vertical() {
                    (*depth_idx, los_idx)
                } else {
                    (los_idx, *depth_idx)
                };
                if depth_iter_idx == 0 || self.tree_heights[i][j] > max_height {
                    visibility_map[i][j] = true;
                    max_height = self.tree_heights[i][j];
                }
            }
        }
        visibility_map
    }

    fn scenic_score_map(&self, direction: &Direction) -> ScenicScoreMap {
        let mut ss_map = vec![vec![0; self.width]; self.height];

        let los_range = if direction.is_vertical() {
            0..self.width
        } else {
            0..self.height
        };

        let depth_range: Vec<usize> = match direction {
            Direction::TB => (0..self.height).collect(),
            Direction::BT => (0..self.height).rev().collect(),
            Direction::LR => (0..self.width).collect(),
            Direction::RL => ((0..self.width).rev()).collect(),
        };

        for los_idx in los_range {
            for depth_range_idx in 0..(depth_range.len() - 1) {
                for lookahead_depth_range_idx in (depth_range_idx + 1)..(depth_range.len()) {
                    let (i, j, i_lookahead, j_lookahead) = if direction.is_vertical() {
                        (
                            depth_range[depth_range_idx],
                            los_idx,
                            depth_range[lookahead_depth_range_idx],
                            los_idx,
                        )
                    } else {
                        (
                            los_idx,
                            depth_range[depth_range_idx],
                            los_idx,
                            depth_range[lookahead_depth_range_idx],
                        )
                    };
                    ss_map[i][j] += 1;
                    // println!("look at {}, {} from {}, {}", i_lookahead, j_lookahead, i, j);
                    if self.tree_heights[i_lookahead][j_lookahead] >= self.tree_heights[i][j] {
                        break;
                    }
                }
            }
        }
        ss_map
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tree_heights.iter() {
            for tree in row {
                write!(f, "{}", tree)?;
            }
            write!(f, "\n")?;
        }
        return Result::Ok(());
    }
}

pub struct TreetopTreeHouse;

impl Solution for TreetopTreeHouse {
    type InputT = Forest;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        Forest::parse(&input_raw)
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut visibility_map = input.empty_visibility_map();
        for direction in [Direction::TB, Direction::LR, Direction::BT, Direction::RL] {
            // println!("{:?}", direction);
            let directional_map = input.visibility_map(&direction);
            // print_visibility_mask(&directional_map);
            for i in 0..input.width {
                for j in 0..input.height {
                    visibility_map[i][j] |= directional_map[i][j];
                }
            }
        }
        visibility_map
            .iter()
            .flatten()
            .map(|is_visible| *is_visible as u32)
            .sum::<u32>()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        let mut scenic_score_map = vec![vec![1; input.width]; input.height];
        for direction in [Direction::TB, Direction::LR, Direction::BT, Direction::RL] {
            // println!("{:?}", direction);
            let directional_map = input.scenic_score_map(&direction);
            // print_2d_vec(&directional_map);
            for i in 0..input.width {
                for j in 0..input.height {
                    scenic_score_map[i][j] *= directional_map[i][j];
                }
            }
        }
        *scenic_score_map.iter().flatten().max().unwrap()
    }
}
