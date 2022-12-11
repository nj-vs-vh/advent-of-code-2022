use std::io::Error;
use std::iter::repeat;
use std::{fmt::Display, fs};

pub fn read_input(day: u8, example: bool) -> Result<String, Error> {
    let filename = if example { "input_example" } else { "input" };
    fs::read_to_string(format!("data/day{:02}/{}.txt", day, filename))
}

pub fn repeated_char(ch: char, count: usize) -> String {
    repeat(ch).take(count).collect::<String>()
}

pub fn ascii_box(content: String, padding: usize, line_width: usize) -> String {
    let lines: Vec<&str> = content.split("\n").collect();
    if lines.len() == 0 {
        return "".to_string();
    }
    let content_width = lines.iter().map(|l| l.len()).max().unwrap();
    let margin_spaces = repeated_char(
        ' ',
        if line_width >= content_width {
            (line_width - content_width) / 2
        } else {
            0
        },
    );

    let mut boxed_lines: Vec<String> = Vec::new();
    boxed_lines.push(format!(
        "{}┌{}┐{}",
        margin_spaces,
        repeated_char('—', content_width + 2 * padding),
        margin_spaces,
    ));
    for _ in 0..padding {
        boxed_lines.push(format!(
            "{}|{}|{}",
            margin_spaces,
            repeated_char(' ', content_width + 2 * padding),
            margin_spaces,
        ));
    }
    for line in lines {
        boxed_lines.push(format!(
            "{}|{}{}{}|{}",
            margin_spaces,
            repeated_char(' ', padding),
            line,
            repeated_char(' ', padding + (content_width - line.len())),
            margin_spaces,
        ));
    }
    for _ in 0..padding {
        boxed_lines.push(format!(
            "{}|{}|{}",
            margin_spaces,
            repeated_char(' ', content_width + 2 * padding),
            margin_spaces,
        ));
    }
    boxed_lines.push(format!(
        "{}└{}┘{}",
        margin_spaces,
        repeated_char('—', content_width + 2 * padding),
        margin_spaces,
    ));
    boxed_lines.join("\n")
}

#[allow(dead_code)]
pub fn print_2d_vec<T: Display>(vm: &Vec<Vec<T>>) {
    for row in vm {
        for item in row {
            print!("{}", item);
        }
        println!("");
    }
    println!("");
}
