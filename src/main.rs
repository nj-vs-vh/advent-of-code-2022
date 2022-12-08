use std::env;
mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day_to_run: u8 = (&args[1])
            .parse()
            .expect("Command line argument must be an integer");
        match day_to_run {
            1 => day01::calories(),
            2 => day02::rock_paper_scissors(),
            3 => day03::rucksacks_reorganization(),
            4 => day04::camp_cleanup(),
            5 => day05::supply_stack(),
            6 => day06::tuning_trouble(),
            7 => day07::no_space_left_on_device(),
            8 => day08::treetop_tree_house(),
            _ => {
                println!("Day {} is not yet implemented", day_to_run)
            }
        }
    }
}
