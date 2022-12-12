use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use image::{codecs::gif::GifEncoder, Delay, Frame, Rgb};

use crate::text_to_image::{text_to_image, CharMatrix};

pub struct CharVisualizationOption {
    pub char: char,
    pub is_bold: bool,
    pub color: Rgb<u8>,
}

pub trait Visualizer {
    fn write_char(&mut self, ch: char);

    fn write_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.write_char(ch);
        }
    }

    fn write_newline(&mut self) {
        self.write_char('\n')
    }

    fn write_line(&mut self, line: &str) {
        self.write_str(&format!("{}\n", line));
    }

    fn end_frame(&mut self);

    fn is_enabled(&self) -> bool;

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOption);
}

pub struct DisabledVisualizer;

impl Visualizer for DisabledVisualizer {
    fn write_char(&mut self, _ch: char) {}

    fn end_frame(&mut self) {}

    fn is_enabled(&self) -> bool {
        false
    }

    fn add_char_visualization_option(&mut self, _opt: CharVisualizationOption) {}
}

const INTERACTIVE_TERMINAL_VISUALIZER_MAX_HISTORY_DEPTH: usize = 1000;

pub struct TerminalVisualizer {
    fps: f32,
    is_interactive: bool,
    prev_displayed_frame_lines: usize,
    frame_buffer: VecDeque<String>,
    opts: Vec<CharVisualizationOption>,
}

impl TerminalVisualizer {
    pub fn new(fps: f32, is_interactive: bool) -> TerminalVisualizer {
        TerminalVisualizer {
            fps,
            is_interactive,
            prev_displayed_frame_lines: 0,
            frame_buffer: VecDeque::with_capacity(
                INTERACTIVE_TERMINAL_VISUALIZER_MAX_HISTORY_DEPTH,
            ),
            opts: Vec::new(),
        }
    }
}

impl Visualizer for TerminalVisualizer {
    fn write_char(&mut self, ch: char) {
        if self.frame_buffer.len() == 0 {
            self.frame_buffer.push_back(String::with_capacity(1000));
        }
        let current_frame_idx = self.frame_buffer.len() - 1;
        match self.opts.iter().find(|o| o.char == ch) {
            Some(char_vis_opt) => {
                let mut style = ansi_term::Style::new();
                style = style.fg(ansi_term::Colour::RGB(
                    char_vis_opt.color[0],
                    char_vis_opt.color[1],
                    char_vis_opt.color[2],
                ));
                if char_vis_opt.is_bold {
                    style = style.bold();
                }
                self.frame_buffer[current_frame_idx]
                    .extend(style.paint(String::from(ch)).to_string().chars())
            }
            None => {
                self.frame_buffer[current_frame_idx].push(ch);
            }
        }
    }

    fn end_frame(&mut self) {
        let mut displayed_frame_idx = self.frame_buffer.len() - 1;

        for _ in 0..self.prev_displayed_frame_lines {
            print!("\x1B[2K\x1B[1A\x1B[2K"); // clear
        }
        print!("{}", self.frame_buffer[displayed_frame_idx]);
        self.prev_displayed_frame_lines = self.frame_buffer[displayed_frame_idx]
            .chars()
            .filter(|c| *c == '\n')
            .count();
        sleep(Duration::from_micros((1e6 / self.fps) as u64));

        // creating new empty frame for the next iteration
        self.frame_buffer.push_back(String::with_capacity(1000));
        if self.frame_buffer.len() > INTERACTIVE_TERMINAL_VISUALIZER_MAX_HISTORY_DEPTH {
            self.frame_buffer.pop_front();
        }
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOption) {
        self.opts.push(opt);
    }
}

pub struct GifVisualizer {
    fps: f32,
    width_px: u32,
    curr_frame: String,
    gif: GifEncoder<File>,
    frame_dimensions: Option<(usize, usize)>,
    frames_since_last_progress_print: u32,
    opts: Vec<CharVisualizationOption>,
}

impl GifVisualizer {
    pub fn new(filename: &str, fps: f32, width_px: u32) -> GifVisualizer {
        GifVisualizer {
            fps,
            width_px,
            curr_frame: String::new(),
            gif: GifEncoder::new_with_speed(
                File::create(filename).expect(&format!("Failed to create file: {}", filename)),
                20,
            ),
            frame_dimensions: None,
            frames_since_last_progress_print: 0,
            opts: Vec::new(),
        }
    }
}

impl Visualizer for GifVisualizer {
    fn write_char(&mut self, ch: char) {
        self.curr_frame.push(ch);
    }

    fn end_frame(&mut self) {
        let mut frame_chars = CharMatrix::new(&self.curr_frame);
        if let Some(dims) = self.frame_dimensions {
            frame_chars.ensure_dimensions(dims.0, dims.1);
        } else {
            self.frame_dimensions = Some(frame_chars.dimensions());
        }

        let image = text_to_image(&frame_chars, self.width_px, 1.0, 0.0);
        self.curr_frame.clear();
        if let Some(img) = image {
            let frame =
                Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(1000, self.fps as u32));
            self.gif
                .encode_frame(frame)
                .expect("Error writing GIF visualization");
        }

        self.frames_since_last_progress_print += 1;
        if self.frames_since_last_progress_print as f32 > self.fps {
            print!(".");
            io::stdout().flush().unwrap();
            self.frames_since_last_progress_print = 0;
        }
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOption) {
        self.opts.push(opt);
    }
}
