mod days;
mod solution;
mod types;
mod utils;

use clap::Parser;
use utils::read_input;

use crate::solution::Solution;

#[derive(Parser, Debug)]
#[command(name = "AoC 2022 solutions")]
#[command(author = "Igor V. <gosha.vaiman@gmail.com>")]
#[command(version = "1.3.1.2")]
struct CliArgs {
    day: u8,

    #[arg(short, long, default_value_t = false)]
    example: bool,

    #[arg(value_enum, default_value_t = types::RunPart::Both)]
    part: types::RunPart,
}

fn main() {
    let args = CliArgs::parse();
    println!("AoC 2022, day {}", args.day);

    let read_input_result = read_input(args.day, args.example);
    if let Err(e) = read_input_result {
        println!("Error reading input file ({})!", e);
        return;
    }
    let input = read_input_result.unwrap();
    let part = args.part;
    match args.day {
        1 => days::day01::CalorieCouting.run(input, part),
        2 => days::day02::RockPaperScissors.run(input, part),
        3 => days::day03::RucksacksReorganization.run(input, part),
        4 => days::day04::CampCleanup.run(input, part),
        5 => days::day05::SupplyStack.run(input, part),
        6 => days::day06::TuningTrouble.run(input, part),
        7 => days::day07::NoSpaceLeftOnDevice.run(input, part),
        8 => days::day08::TreetopTreeHouse.run(input, part),
        _ => {
            println!("Solution is not yet implemented");
        }
    }
}
