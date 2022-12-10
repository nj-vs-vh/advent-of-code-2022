use std::fmt::Display;

pub trait Solution {
    type InputT: Clone;
    type OutputT: Display;

    fn parse_input(&self, input_raw: String) -> Self::InputT;

    fn solve_pt1(&self, input: Self::InputT) -> Self::OutputT;

    fn solve_pt2(&self, input: Self::InputT) -> Self::OutputT;
}
