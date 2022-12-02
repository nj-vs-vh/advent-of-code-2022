use crate::utils::read_input;

pub fn calories() {
    let input = read_input(1, false);

    let mut elve_calories: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.split("\n").map(|s| s.parse::<u32>().unwrap()).sum())
        .collect();
    // println!("{:?}", elve_calories);

    println!("pt1: {}", elve_calories.iter().max().unwrap());

    elve_calories.sort();
    elve_calories.reverse();
    println!("pt2: {}", elve_calories[0..3].iter().sum::<u32>());
}
