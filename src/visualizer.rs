use std::{
    fs::File,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use image::{codecs::gif::GifEncoder, Delay, Frame, Rgb};

use crate::text_to_image::{text_to_image, CharMatrix};

pub struct CharVisualizationOptions {
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

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOptions);
}

pub struct DisabledVisualizer;

impl Visualizer for DisabledVisualizer {
    fn write_char(&mut self, _ch: char) {}

    fn end_frame(&mut self) {}

    fn is_enabled(&self) -> bool {
        false
    }

    fn add_char_visualization_option(&mut self, _opt: CharVisualizationOptions) {}
}

pub struct TerminalVisualizer {
    fps: f32,
    prev_frame_lines: usize,
    curr_frame_buffer: String,
    opts: Vec<CharVisualizationOptions>,
}

impl TerminalVisualizer {
    pub fn new(fps: f32) -> TerminalVisualizer {
        TerminalVisualizer {
            fps,
            prev_frame_lines: 0,
            curr_frame_buffer: String::with_capacity(4096),
            opts: Vec::new(),
        }
    }
}

impl Visualizer for TerminalVisualizer {
    fn write_char(&mut self, ch: char) {
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
                self.curr_frame_buffer
                    .extend(style.paint(String::from(ch)).to_string().chars())
            }
            None => {
                self.curr_frame_buffer.push(ch);
            }
        }
    }

    fn end_frame(&mut self) {
        for _ in 0..self.prev_frame_lines {
            print!("\x1B[2K\x1B[1A\x1B[2K"); // clear line
        }
        print!("{}", self.curr_frame_buffer);
        self.prev_frame_lines = self
            .curr_frame_buffer
            .chars()
            .filter(|c| *c == '\n')
            .count();
        self.curr_frame_buffer.clear();
        sleep(Duration::from_micros((1e6 / self.fps) as u64));
    }

    fn is_enabled(&self) -> bool {
        true
    }

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOptions) {
        self.opts.push(opt);
    }
}

pub struct GifVisualizer {
    fps: f32,
    width_px: u32,
    curr_frame_buffer: String,
    gif: GifEncoder<File>,
    frame_dimensions: Option<(usize, usize)>,
    frames_since_last_progress_print: u32,
    opts: Vec<CharVisualizationOptions>,
}

impl GifVisualizer {
    pub fn new(filename: &str, fps: f32, width_px: u32) -> GifVisualizer {
        GifVisualizer {
            fps,
            width_px,
            curr_frame_buffer: String::new(),
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
        self.curr_frame_buffer.push(ch);
    }

    fn end_frame(&mut self) {
        let mut frame_chars = CharMatrix::new(&self.curr_frame_buffer);
        if let Some(dims) = self.frame_dimensions {
            frame_chars.ensure_dimensions(dims.0, dims.1);
        } else {
            self.frame_dimensions = Some(frame_chars.dimensions());
        }

        let image = text_to_image(&frame_chars, self.width_px, 1.0, 0.0);
        self.curr_frame_buffer.clear();
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

    fn add_char_visualization_option(&mut self, opt: CharVisualizationOptions) {
        self.opts.push(opt);
    }
}
