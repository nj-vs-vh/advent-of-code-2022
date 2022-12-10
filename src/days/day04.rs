use crate::solution::Solution;
use crate::visualizer::Visualizer;

pub struct Range(u32, u32);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.1 < other.0 || self.0 > other.1)
    }
}

fn parse_range(s: &str) -> Range {
    let mut it = s.split("-").map(|d| d.parse::<u32>().unwrap());
    Range(it.next().unwrap(), it.next().unwrap())
}

fn parse_ranges(line: &str) -> (Range, Range) {
    let mut it = line.split(",").map(parse_range);
    (it.next().unwrap(), it.next().unwrap())
}
pub struct CampCleanup;

impl Solution for CampCleanup {
    type InputT = Vec<(Range, Range)>;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw.lines().map(parse_ranges).collect()
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .iter()
            .map(|(r1, r2)| (r1.contains(&r2) || r2.contains(&r1)) as u32)
            .sum::<u32>()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        input
            .iter()
            .map(|(r1, r2)| (r1.overlaps(&r2)) as u32)
            .sum::<u32>()
    }
}
