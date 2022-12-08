use std::{fmt::Display, fs};

pub fn read_input(day: u8, test: bool) -> String {
    let filename = if test { "input_test" } else { "input" };
    fs::read_to_string(format!("data/day{:02}/{}.txt", day, filename))
        .expect("Can't read input file")
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
