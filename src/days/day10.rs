use crate::solution::Solution;

#[derive(Debug)]
pub enum Op {
    Addx(i32),
    Noop,
}

pub struct OpStream {
    lines: Vec<String>,
}

impl From<String> for OpStream {
    fn from(input: String) -> Self {
        OpStream {
            lines: input.lines().map(|l| l.to_string()).rev().collect(),
        }
    }
}

impl Iterator for OpStream {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        let opstr = self.lines.pop()?;
        let mut words_iter = opstr.split(" ");
        match words_iter.next()? {
            "addx" => Some(Op::Addx(
                words_iter
                    .next()?
                    .parse()
                    .expect("Addx must be followed by a number"),
            )),
            "noop" => Some(Op::Noop),
            _ => {
                panic!("Unknown command {}", opstr);
            }
        }
    }
}

pub struct CathodeRayTube;

impl Solution for CathodeRayTube {
    type InputT = OpStream;
    type OutputT = i32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        OpStream::from(input_raw)
    }

    fn solve_pt1(
        &self,
        input: Self::InputT,
        visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let mut x: i32 = 1;
        let mut cycle: u32 = 1;
        let mut strength: i32 = 0;

        for (op_idx, op) in input.enumerate() {
            visualizer.write_line(&format!(
                "Running op {} {:?} [x = {}, cycle = {}]",
                op_idx, op, x, cycle
            ));
            let (new_cycle, new_x) = match op {
                Op::Noop => (cycle + 1, x),
                Op::Addx(v) => (cycle + 2, x + v),
            };

            let maybe_recording_cycle =
                (cycle..new_cycle).find(|c| *c >= 20 && ((c - 20) % 40) == 0);

            if let Some(recording_cycle) = maybe_recording_cycle {
                let strength_add = x * (recording_cycle as i32);
                strength += strength_add;
                visualizer.write_line(&format!(
                    "During this operation we record signal strength {} (now {})!",
                    strength_add, strength
                ));
            }
            x = new_x;
            cycle = new_cycle;
            visualizer.end_frame();
        }
        strength
    }

    fn solve_pt2(
        &self,
        input: Self::InputT,
        visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let mut x_curr: i32 = 1;
        let mut cycle_curr: u32 = 0;

        const LINES: usize = 6;
        const PIXELS: usize = 40;
        let mut screen = [[false; PIXELS]; LINES];

        for (_, op) in input.enumerate() {
            let (cycle_after, x_after) = match op {
                Op::Noop => (cycle_curr + 1, x_curr),
                Op::Addx(v) => (cycle_curr + 2, x_curr + v),
            };

            for cycle in cycle_curr..cycle_after {
                let pixel = cycle as i32 % PIXELS as i32;
                if (pixel - x_curr).abs() <= 1 {
                    screen[cycle as usize / PIXELS][pixel as usize] = true;
                }
                visualizer.write_line(&format!(
                    "cycle {}, x = {}, {} {:?}",
                    cycle,
                    x_curr,
                    if cycle == cycle_curr {
                        "starting"
                    } else {
                        "executing"
                    },
                    op
                ));
                for i in 0..LINES {
                    for j in 0..PIXELS {
                        visualizer.write_char(if screen[i][j] { '#' } else { '.' });
                    }
                    visualizer.write_newline();
                }
                visualizer.end_frame();
            }

            x_curr = x_after;
            cycle_curr = cycle_after;
        }
        0
    }
}
