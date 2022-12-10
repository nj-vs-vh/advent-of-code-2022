use std::{fmt::Display, time::Instant};

use crate::types::RunPart;
use crate::utils::ascii_box;
use crate::visualizer::Visualizer;

pub trait Solution {
    type InputT;
    type OutputT: Display;

    fn run(&self, input_raw: String, part: RunPart, mut visualizer: Box<dyn Visualizer>) {
        let input = self.parse_input(input_raw.clone());
        let input_clone = self.parse_input(input_raw);

        if part == RunPart::Pt1 || part == RunPart::Both {
            let start_pt1 = Instant::now();
            let output_pt1 = self.solve_pt1(input, visualizer.as_mut());
            println!(
                "\nPart 1 solution (took {:.3} msec):\n{}",
                start_pt1.elapsed().as_secs_f32() * 1000.0,
                ascii_box(format!("{}", output_pt1), 1, 35)
            );
        }
        if part == RunPart::Pt2 || part == RunPart::Both {
            let start_pt2 = Instant::now();
            let output_pt2 = self.solve_pt2(input_clone, visualizer.as_mut());
            println!(
                "\nPart 2 solution (took {:.3} msec):\n{}",
                start_pt2.elapsed().as_secs_f32() * 1000.0,
                ascii_box(format!("{}", output_pt2), 1, 35)
            );
        }
    }

    fn parse_input(&self, input_raw: String) -> Self::InputT;

    fn solve_pt1(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT;

    fn solve_pt2(&self, input: Self::InputT, visualizer: &mut dyn Visualizer) -> Self::OutputT;
}
