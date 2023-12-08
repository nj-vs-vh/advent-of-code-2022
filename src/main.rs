mod color;
mod days;
mod solution;
mod text_to_image;
mod types;
mod utils;
mod visualizer;

use std::path::PathBuf;

use clap::Parser;
use utils::read_input;

use crate::{
    solution::Solution,
    visualizer::{DisabledVisualizer, GifVisualizer, TerminalVisualizer, Visualizer},
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

    #[arg(long, default_value_t = 30.0)]
    fps: f32,

    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(long, value_name = "FILE")]
    gif: Option<PathBuf>,

    #[arg(long, default_value_t = 800)]
    gif_width: u32,
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
        true => {
            if let Some(gif_path) = args.gif {
                Box::new(GifVisualizer::new(
                    gif_path.to_str().unwrap(),
                    args.fps,
                    args.gif_width,
                ))
            } else {
                Box::new(TerminalVisualizer::new(args.fps, args.interactive))
            }
        }
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
        9 => days::day09::RopeBridge.run(input, part, vis),
        10 => days::day10::CathodeRayTube.run(input, part, vis),
        11 => days::day11::MonkeyInTheMiddle.run(input, part, vis),
        12 => days::day12::HillClimbingAlgorithm.run(input, part, vis),
        13 => days::day13::DistressSignal.run(input, part, vis),
        14 => days::day14::RegolithReservoir.run(input, part, vis),
        15 => days::day15::BeaconExclusionZone.run(input, part, vis),
        16 => days::day16::ProboscideaVolcanium.run(input, part, vis),
        _ => {
            println!("Solution is not yet implemented");
        }
    }
}
