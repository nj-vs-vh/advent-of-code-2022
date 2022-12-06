use crate::utils::read_input;

const DAY: u8 = 6;

fn encode_char(ch: &char) -> u32 {
    let char_code = (*ch as u32) - 97;
    (2 as u32).pow(char_code)
}

// const BUFFER_SIZE: usize = 4; // for pt1
const BUFFER_SIZE: usize = 14; // for pt2

struct Buffer {
    buf: [u32; BUFFER_SIZE],
    write_idx: usize,
    seen_values: u32,
    is_filled: bool,
}

impl Buffer {
    fn new() -> Buffer {
        return Buffer {
            buf: [0; BUFFER_SIZE],
            write_idx: 0,
            seen_values: 0,
            is_filled: false,
        };
    }

    fn update(&mut self, value: u32) {
        self.buf[self.write_idx] = value;
        self.write_idx = (self.write_idx + 1) % BUFFER_SIZE;
        self.seen_values += 1;
        if !self.is_filled && self.write_idx == 0 {
            // marking the buffer filled when we looped over to write pos = 0
            self.is_filled = true;
        }
        // println!("{:?}", self.buf);
    }

    fn all_distinct(&self) -> bool {
        if !self.is_filled {
            return false;
        }
        let mut acc: u32 = 0;
        for value in self.buf.iter() {
            if value & acc > 0 {
                return false;
            }
            acc = acc | value
        }
        return true;
    }
}

pub fn tuning_trouble() {
    let input = read_input(DAY, false);

    let mut buffer = Buffer::new();
    for character in input.chars() {
        buffer.update(encode_char(&character));
        if buffer.all_distinct() {
            break;
        }
    }
    println!(
        "first all-distinct sequence of {} chars found at: {}",
        BUFFER_SIZE, buffer.seen_values
    )
}