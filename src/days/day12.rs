use std::collections::HashSet;

use crate::{
    color::get_rgb_pixel, solution::Solution, types::Coords, visualizer::CharVisualizationOption,
    visualizer::Visualizer,
};

#[derive(Debug)]
pub struct HeightMap {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: Coords<usize>,
    end: Coords<usize>,
}

impl HeightMap {
    fn at(&self, c: &Coords<usize>) -> u8 {
        self.map[c.y][c.x]
    }

    fn steps_from(&self, c: &Coords<usize>, is_uphill: bool) -> Vec<Coords<usize>> {
        let mut res: Vec<Coords<usize>> = Vec::new();
        let current_height = self.map[c.y][c.x];

        let is_step_ok: Box<dyn Fn(u8) -> bool> = if is_uphill {
            Box::new(|height_step: u8| height_step <= current_height + 1)
        } else {
            Box::new(|height_step: u8| height_step >= current_height - 1)
        };

        let i = c.y as i32;
        let j = c.x as i32;
        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let i_step = i + di;
            let j_step = j + dj;
            if i_step >= 0
                && i_step < self.height as i32
                && j_step >= 0
                && j_step < self.width as i32
                && is_step_ok(self.map[i_step as usize][j_step as usize])
            {
                res.push(Coords {
                    y: i_step as usize,
                    x: j_step as usize,
                });
            }
        }
        res
    }
}

fn setup_visualizer(vis: &mut dyn Visualizer) {
    for h in 0..26 {
        vis.add_char_visualization_option(CharVisualizationOption {
            char: (h as u8 + 97) as char,
            is_bold: false,
            color: get_rgb_pixel(0, 0, (40.0 + 60.0 * h as f32 / 26.0) as u8),
        })
    }
    for ch in ['[', ']'] {
        vis.add_char_visualization_option(CharVisualizationOption {
            char: ch,
            is_bold: true,
            color: get_rgb_pixel(0, 100, 50),
        })
    }
    for ch in ['(', ')'] {
        vis.add_char_visualization_option(CharVisualizationOption {
            char: ch,
            is_bold: true,
            color: get_rgb_pixel(35, 100, 50),
        })
    }
}

fn visualize(
    vis: &mut dyn Visualizer,
    hm: &HeightMap,
    visited: &HashSet<Coords<usize>>,
    current: &HashSet<Coords<usize>>,
) {
    if !vis.is_enabled() {
        return;
    }
    for i in 0..hm.height {
        for j in 0..hm.width {
            let c = Coords { y: i, x: j };
            let ch = (hm.map[i][j] + 97) as char;
            if current.contains(&c) {
                vis.write_str(&format!("[{}]", ch));
            } else if visited.contains(&c) {
                vis.write_str(&format!("({})", ch));
            } else {
                vis.write_str(&format!(" {} ", ch));
            }
        }
        vis.write_newline();
    }
    vis.end_frame();
}

pub struct HillClimbingAlgorithm;

impl Solution for HillClimbingAlgorithm {
    type InputT = HeightMap;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut start: Option<Coords<usize>> = None;
        let mut end: Option<Coords<usize>> = None;
        for (i, line) in input_raw.lines().enumerate() {
            map.push(Vec::new());
            for (j, ch) in line.chars().enumerate() {
                map.last_mut().unwrap().push(match ch {
                    'S' => {
                        start = Some(Coords { y: i, x: j });
                        0
                    }
                    'E' => {
                        end = Some(Coords { y: i, x: j });
                        25
                    }
                    _ => ch as u8 - 97,
                })
            }
        }
        let width = map.iter().next().unwrap().len();
        let height = map.len();
        HeightMap {
            map,
            start: start.unwrap(),
            end: end.unwrap(),
            width,
            height,
        }
    }

    fn solve_pt1(
        &self,
        input: Self::InputT,
        vis: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        setup_visualizer(vis);
        let mut visited: HashSet<Coords<usize>> = HashSet::new();
        let mut current: HashSet<Coords<usize>> = HashSet::from([input.start]);

        let mut steps_so_far = 0;
        loop {
            visualize(vis, &input, &visited, &current);
            let mut next: HashSet<Coords<usize>> = HashSet::new();
            for c in current.iter() {
                for c_next in input.steps_from(&c, true) {
                    if c_next == input.end {
                        return steps_so_far + 1;
                    } else if visited.contains(&c_next) || current.contains(&c_next) {
                        continue;
                    } else {
                        next.insert(c_next);
                    }
                }
            }
            visited.extend(current.drain());
            current = next;
            steps_so_far += 1;
        }
    }

    fn solve_pt2(
        &self,
        input: Self::InputT,
        vis: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        setup_visualizer(vis);
        let mut visited: HashSet<Coords<usize>> = HashSet::new();
        let mut current: HashSet<Coords<usize>> = HashSet::from([input.end]);

        let mut steps_so_far = 0;
        loop {
            visualize(vis, &input, &visited, &current);
            let mut next: HashSet<Coords<usize>> = HashSet::new();
            for c in current.iter() {
                for c_next in input.steps_from(&c, false) {
                    if input.at(&c_next) == 0 {
                        return steps_so_far + 1;
                    } else if visited.contains(&c_next) || current.contains(&c_next) {
                        continue;
                    } else {
                        next.insert(c_next);
                    }
                }
            }
            visited.extend(current.drain());
            current = next;
            steps_so_far += 1;
        }
    }
}
