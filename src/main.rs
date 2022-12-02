mod day01;
use std::env;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day_to_run: u8 = (&args[1])
            .parse()
            .expect("Command line argument must be an integer");
        match day_to_run {
            1 => day01::calories(),
            _ => {
                println!("Day {} is not yet implemented", day_to_run)
            }
        }
    } else {
        // new day will be here...
    }
}
