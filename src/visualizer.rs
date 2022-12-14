use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use image::{codecs::gif::GifEncoder, Delay, Frame, Rgb};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    text_to_image::{text_to_image, CharMatrix},
    types::Coords,
};

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
    prev_displayed_lines: usize,
    frame_buffer: VecDeque<String>,
    opts: Vec<CharVisualizationOption>,
    top_left_offset: Coords<usize>,
}

impl TerminalVisualizer {
    pub fn new(fps: f32, is_interactive: bool) -> TerminalVisualizer {
        TerminalVisualizer {
            fps,
            is_interactive,
            prev_displayed_lines: 0,
            frame_buffer: VecDeque::with_capacity(
                INTERACTIVE_TERMINAL_VISUALIZER_MAX_HISTORY_DEPTH,
            ),
            opts: Vec::new(),
            top_left_offset: Coords::origin(),
        }
    }
}

impl TerminalVisualizer {
    fn apply_opts(&self, ch: char) -> Option<String> {
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
                Some(style.paint(String::from(ch)).to_string())
            }
            None => None,
        }
    }
}

impl Visualizer for TerminalVisualizer {
    fn write_char(&mut self, ch: char) {
        if self.frame_buffer.len() == 0 {
            self.frame_buffer.push_back(String::with_capacity(1000));
        }
        let current_frame_idx = self.frame_buffer.len() - 1;
        self.frame_buffer[current_frame_idx].push(ch);
    }

    fn end_frame(&mut self) {
        let mut displayed_frame_idx = self.frame_buffer.len() - 1;

        loop {
            for _ in 0..self.prev_displayed_lines {
                print!("\x1B[2K\x1B[1A\x1B[2K"); // clear
            }

            let displayed_frame = &self.frame_buffer[displayed_frame_idx];

            let (terminal_width, terminal_height) = termion::terminal_size().unwrap_or((80, 80));
            let display_width = terminal_width as usize;
            let display_height = if terminal_height > 20 {
                terminal_height as usize - 15
            } else {
                terminal_height as usize
            };
            let displayed_frame_lines: Vec<String> = displayed_frame
                .lines()
                .skip(self.top_left_offset.y)
                .take(display_height)
                .map(|l| {
                    l.chars()
                        .skip(self.top_left_offset.x)
                        .take(display_width)
                        .collect()
                })
                .collect();

            let mut to_print: String = String::new();
            for ch in displayed_frame_lines.join("\n").chars() {
                if let Some(with_opts) = self.apply_opts(ch) {
                    to_print.extend(with_opts.chars());
                } else {
                    to_print.push(ch);
                }
            }
            print!("{}", to_print);

            self.prev_displayed_lines = displayed_frame_lines.len();

            if !self.is_interactive {
                sleep(Duration::from_micros((1e6 / self.fps) as u64));
                break;
            } else {
                // user controls input loop
                println!(
                    "\ninteractive mode (frame {} / {})\nh, l - forward and backwards\n_, $ - jump to the first and last frame\narrows - pan displayed frame portion\nq - exit interactive mode",
                    displayed_frame_idx + 1,
                    self.frame_buffer.len()
                );
                let stdin = io::stdin();
                let mut stdout = io::stdout().into_raw_mode().unwrap();
                let mut generate_next_frame = false;
                for c in stdin.keys() {
                    write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

                    match c.unwrap() {
                        Key::Char('h') => {
                            if displayed_frame_idx > 0 {
                                displayed_frame_idx -= 1;
                                break;
                            }
                        }
                        Key::Char('l') | Key::Char(' ') => {
                            if displayed_frame_idx < self.frame_buffer.len() - 1 {
                                displayed_frame_idx += 1;
                            } else {
                                generate_next_frame = true;
                            }
                            break;
                        }
                        Key::Char('$') => {
                            displayed_frame_idx = self.frame_buffer.len() - 1;
                            break;
                        }
                        Key::Char('_') => {
                            displayed_frame_idx = 0;
                            break;
                        }
                        Key::Left => {
                            if self.top_left_offset.x > 0 {
                                self.top_left_offset.x -= 1;
                            }
                            break;
                        }
                        Key::Right => {
                            self.top_left_offset.x += 1;
                            break;
                        }
                        Key::Up => {
                            if self.top_left_offset.y > 0 {
                                self.top_left_offset.y -= 1;
                            }
                            break;
                        }
                        Key::Down => {
                            self.top_left_offset.y += 1;
                            break;
                        }
                        Key::Char('q') => {
                            self.is_interactive = false;
                            generate_next_frame = true;
                            break;
                        }
                        _ => {}
                    }
                    stdout.flush().unwrap();
                }

                for _ in 0..5 {
                    // clearing prompt
                    print!("\x1B[2K\x1B[1A\x1B[2K");
                }
                if generate_next_frame {
                    break;
                }
            }
        }

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

        let image = text_to_image(&frame_chars, self.width_px, 1.0, 0.0, &self.opts);
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
