use std::str::FromStr;
use std::{cmp::max, collections::HashSet};

use itertools::Itertools;
use strum_macros::{Display, EnumString};

use crate::{solution::Solution, types::Coords};

#[derive(Debug, EnumString, Display)]
pub enum Direction {
    L,
    R,
    U,
    D,
}

impl Direction {
    fn delta(&self) -> Coords<i32> {
        match self {
            Direction::L => Coords { x: -1, y: 0 },
            Direction::R => Coords { x: 1, y: 0 },
            Direction::U => Coords { x: 0, y: 1 },
            Direction::D => Coords { x: 0, y: -1 },
        }
    }
}

pub struct RopeBridge;

impl Solution for RopeBridge {
    type InputT = Vec<(Direction, usize)>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut res: Self::InputT = Vec::new();
        for line in input_raw.lines() {
            let mut parts_iter = line.split(" ");
            let direction = Direction::from_str(parts_iter.next().unwrap()).unwrap();
            let amount = parts_iter.next().unwrap().parse::<usize>().unwrap();
            res.push((direction, amount));
        }
        res
    }

    fn solve_pt1(
        &self,
        input: Self::InputT,
        visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let (mut vis_hh, mut vis_hw) = (10, 10);
        let mut head = Coords::<i32> { x: 0, y: 0 };
        let mut tail = Coords::<i32> { x: 0, y: 0 };
        let mut tail_positions: HashSet<Coords<i32>> = HashSet::new();

        for (direction, amount) in input {
            for _ in 0..amount {
                head += direction.delta();
                let mut delta = head - tail;
                if delta.x.abs() > 1 || delta.y.abs() > 1 {
                    delta.x = num::clamp(delta.x, -1, 1);
                    delta.y = num::clamp(delta.y, -1, 1);
                    tail += delta;
                }
                tail_positions.insert(tail.clone());

                if visualizer.is_enabled() {
                    vis_hw = max(max(head.x.abs(), tail.x.abs()), vis_hw);
                    vis_hh = max(max(head.y.abs(), tail.y.abs()), vis_hh);
                    for y in (-vis_hh..=vis_hh).rev() {
                        for x in -vis_hw..=vis_hw {
                            let c = Coords { x, y };
                            visualizer.write_char(if c == head {
                                'H'
                            } else if c == tail {
                                'T'
                            } else if c.is_origin() {
                                's'
                            } else {
                                '.'
                            })
                        }
                        visualizer.write_newline();
                    }
                    visualizer.end_frame();
                }
            }
        }

        tail_positions.len()
    }

    fn solve_pt2(
        &self,
        input: Self::InputT,
        visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        const KNOTS: usize = 10;

        const VIS_HALFSIDE: i32 = 8;
        let mut vis_center: Coords<i32> = Coords::origin();

        let mut rope = [Coords::origin(); KNOTS];
        let mut tail_positions: HashSet<Coords<i32>> = HashSet::new();

        for (direction, amount) in input {
            for _ in 0..amount {
                let mut current_delta = direction.delta();
                for knot_idx in 0..KNOTS - 1 {
                    rope[knot_idx] += current_delta;
                    current_delta = rope[knot_idx] - rope[knot_idx + 1];
                    if current_delta.x.abs() > 1 || current_delta.y.abs() > 1 {
                        current_delta.x = num::clamp(current_delta.x, -1, 1);
                        current_delta.y = num::clamp(current_delta.y, -1, 1);
                    } else {
                        current_delta = Coords::origin();
                        break; // if a knot is stationary, the rest of the rope is too
                    }
                }
                rope[KNOTS - 1] += current_delta;
                tail_positions.insert(rope[KNOTS - 1].clone());

                if visualizer.is_enabled() {
                    vis_center.x = rope.iter().map(|c| c.x).sum::<i32>() / (KNOTS as i32);
                    vis_center.y = rope.iter().map(|c| c.y).sum::<i32>() / (KNOTS as i32);
                    for y in ((vis_center.y - VIS_HALFSIDE)..=(vis_center.y + VIS_HALFSIDE)).rev() {
                        for x in (vis_center.x - VIS_HALFSIDE)..=(vis_center.x + VIS_HALFSIDE) {
                            let c = Coords { x, y };
                            let knot_idx = rope.iter().find_position(|k| **k == c);
                            if let Some((idx, _)) = knot_idx {
                                let default = format!("{}", idx);
                                visualizer.write(if idx == 0 {
                                    "H"
                                } else if idx == KNOTS {
                                    "T"
                                } else {
                                    &default
                                })
                            } else {
                                const GRID_LINES_EACH: i32 = 10;
                                if x % GRID_LINES_EACH == 0 {
                                    if y % GRID_LINES_EACH == 0 {
                                        visualizer.write("+");
                                    } else {
                                        visualizer.write("|")
                                    }
                                } else {
                                    if y % GRID_LINES_EACH == 0 {
                                        visualizer.write("-")
                                    } else {
                                        visualizer.write(" ")
                                    }
                                }
                            }
                        }
                        visualizer.write_newline();
                    }
                    visualizer.end_frame();
                }
            }
        }

        tail_positions.len()
    }
}
