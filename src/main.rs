mod days;
mod solution;
mod utils;

use clap::Parser;
use clap::ValueEnum;
use days::get_solution;
use solution::Solution;
use std::time::Instant;
use utils::read_input;

use crate::utils::ascii_box;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum RunPart {
    Pt1,
    Pt2,
    Both,
}

#[derive(Parser, Debug)]
#[command(name = "AoC 2022 solutions")]
#[command(author = "Igor V. <gosha.vaiman@gmail.com>")]
#[command(version = "1.3.1.2")]
struct CliArgs {
    day: u8,

    #[arg(short, long, default_value_t = false)]
    example: bool,

    #[arg(value_enum, default_value_t = RunPart::Both)]
    part: RunPart,
}

fn main() {
    let args = CliArgs::parse();
    println!("AoC 2022, day {}", args.day);

    let maybe_solution = get_solution(&args.day);
    if let None = maybe_solution {
        println!("Solution is not yet implemented");
        return;
    }
    let solution = maybe_solution.unwrap();

    let read_input_result = read_input(args.day, args.example);
    if let Err(e) = read_input_result {
        println!("Error reading input file ({})!", e);
        return;
    }
    let input_string = read_input_result.unwrap();

    let input = solution.parse_input(input_string);
    let input_c = input.clone();

    if args.part == RunPart::Pt1 || args.part == RunPart::Both {
        let start_pt1 = Instant::now();
        let output_pt1 = solution.solve_pt1(input);
        println!(
            "\nPart 1 solution (took {:.3} msec):\n{}",
            start_pt1.elapsed().as_secs_f32() * 1000.0,
            ascii_box(format!("{}", output_pt1), 1, 35)
        );
    }
    if args.part == RunPart::Pt2 || args.part == RunPart::Both {
        let start_pt2 = Instant::now();
        let output_pt2 = solution.solve_pt2(input_c);
        println!(
            "\nPart 2 solution (took {:.3} msec):\n{}",
            start_pt2.elapsed().as_secs_f32() * 1000.0,
            ascii_box(format!("{}", output_pt2), 1, 35)
        );
    }
}
