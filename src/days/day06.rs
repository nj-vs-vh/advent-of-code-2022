use crate::visualizer::Visualizer;
use std::str::Chars;

use crate::solution::Solution;

type EncodedChar = u32;

fn encode_char(ch: &char) -> EncodedChar {
    let char_code = (*ch as u32) - 97;
    (2 as EncodedChar).pow(char_code)
}

const MAX_BUFFER_SIZE: usize = 14; // for pt2

struct Buffer {
    buf: [EncodedChar; MAX_BUFFER_SIZE],
    size: usize,
    write_idx: usize,
    seen_values: u32,
    is_filled: bool,
}

impl Buffer {
    fn new(size: usize) -> Buffer {
        if size > MAX_BUFFER_SIZE {
            panic!("Too large buffer!");
        }
        return Buffer {
            buf: [0; MAX_BUFFER_SIZE],
            size,
            write_idx: 0,
            seen_values: 0,
            is_filled: false,
        };
    }

    fn update(&mut self, value: EncodedChar) {
        self.buf[self.write_idx] = value;
        self.write_idx = (self.write_idx + 1) % self.size;
        self.seen_values += 1;
        if !self.is_filled && self.write_idx == 0 {
            self.is_filled = true;
        }
        // println!("{:?}", self.buf);
    }

    fn all_distinct(&self) -> bool {
        if !self.is_filled {
            return false;
        }
        let mut acc: EncodedChar = 0;
        for value in self.buf {
            if value & acc > 0 {
                return false;
            }
            acc = acc | value
        }
        return true;
    }

    fn find_distinct(&mut self, characters: Chars) -> Option<u32> {
        for character in characters {
            self.update(encode_char(&character));
            if self.all_distinct() {
                return Some(self.seen_values);
            }
        }
        None
    }
}

pub struct TuningTrouble;

impl Solution for TuningTrouble {
    type InputT = String;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        input_raw
    }

    fn solve_pt1(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        Buffer::new(4).find_distinct(input.chars()).unwrap()
    }

    fn solve_pt2(&self, input: Self::InputT, _visualizer: &mut dyn Visualizer) -> Self::OutputT {
        Buffer::new(14).find_distinct(input.chars()).unwrap()
    }
}
