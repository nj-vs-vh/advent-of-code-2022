use std::fmt::Display;

#[allow(unused_imports)]
use crate::utils::{print_2d_vec, read_input};

const DAY: u8 = 8;

#[derive(Debug)]
struct Forest {
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

type ScenicScoreMap = Vec<Vec<usize>>;

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

pub fn treetop_tree_house() {
    let input = read_input(DAY, false);
    let forest = Forest::parse(&input);
    // println!("{}", forest);

    let mut visibility_map = forest.empty_visibility_map();
    for direction in [Direction::TB, Direction::LR, Direction::BT, Direction::RL] {
        // println!("{:?}", direction);
        let directional_map = forest.visibility_map(&direction);
        // print_visibility_mask(&directional_map);
        for i in 0..forest.width {
            for j in 0..forest.height {
                visibility_map[i][j] |= directional_map[i][j];
            }
        }
    }
    println!(
        "pt1: {}",
        visibility_map
            .iter()
            .flatten()
            .map(|is_visible| *is_visible as u32)
            .sum::<u32>()
    );

    let mut scenic_score_map = vec![vec![1; forest.width]; forest.height];
    for direction in [Direction::TB, Direction::LR, Direction::BT, Direction::RL] {
        // println!("{:?}", direction);
        let directional_map = forest.scenic_score_map(&direction);
        // print_2d_vec(&directional_map);
        for i in 0..forest.width {
            for j in 0..forest.height {
                scenic_score_map[i][j] *= directional_map[i][j];
            }
        }
    }
    println!("pt1: {}", scenic_score_map.iter().flatten().max().unwrap())
}
