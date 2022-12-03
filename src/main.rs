use std::env;
mod utils;

mod day01;
mod day02;
mod day03;

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
            _ => {
                println!("Day {} is not yet implemented", day_to_run)
            }
        }
    }
}
