mod days;
mod solution;
mod types;
mod utils;
mod visualizer;

use clap::Parser;
use utils::read_input;

use crate::{
    solution::Solution,
    visualizer::{DisabledVisualizer, TerminalVisualizer, Visualizer},
};

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

    #[arg(short, long, default_value_t = false)]
    visualize: bool,
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

    let vis: Box<dyn Visualizer> = match args.visualize {
        true => Box::new(TerminalVisualizer::new(1)),
        false => Box::new(DisabledVisualizer {}),
    };

    match args.day {
        1 => days::day01::CalorieCouting.run(input, part, vis),
        2 => days::day02::RockPaperScissors.run(input, part, vis),
        3 => days::day03::RucksacksReorganization.run(input, part, vis),
        4 => days::day04::CampCleanup.run(input, part, vis),
        5 => days::day05::SupplyStack.run(input, part, vis),
        6 => days::day06::TuningTrouble.run(input, part, vis),
        7 => days::day07::NoSpaceLeftOnDevice.run(input, part, vis),
        8 => days::day08::TreetopTreeHouse.run(input, part, vis),
        _ => {
            println!("Solution is not yet implemented");
        }
    }
}
