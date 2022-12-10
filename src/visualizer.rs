use std::{thread::sleep, time::Duration};

pub trait Visualizer {
    fn write(&mut self, s: &str);

    fn write_char(&mut self, ch: &char) {
        self.write(&ch.to_string());
    }

    fn write_line(&mut self, line: &str) {
        self.write(&format!("{}\n", line));
    }

    fn end_frame(&mut self);
}

pub struct DisabledVisualizer;

impl Visualizer for DisabledVisualizer {
    fn write(&mut self, _: &str) {}

    fn end_frame(&mut self) {}
}

pub struct TerminalVisualizer {
    frame_delay: Duration,
    prev_frame_lines: usize,
    curr_frame_buffer: String,
}

impl TerminalVisualizer {
    pub fn new(delay_sec: u32) -> TerminalVisualizer {
        TerminalVisualizer {
            frame_delay: Duration::from_secs(delay_sec as u64),
            prev_frame_lines: 0,
            curr_frame_buffer: String::with_capacity(4096),
        }
    }
}

impl Visualizer for TerminalVisualizer {
    fn write(&mut self, s: &str) {
        self.curr_frame_buffer.extend(s.chars());
    }

    fn end_frame(&mut self) {
        for _ in 0..self.prev_frame_lines {
            print!("\x1B[2K\x1B[1A"); // clear line
        }
        print!("{}", self.curr_frame_buffer);
        self.prev_frame_lines = self
            .curr_frame_buffer
            .chars()
            .filter(|c| *c == '\n')
            .count();
        self.curr_frame_buffer.clear();
        sleep(self.frame_delay);
    }
}
