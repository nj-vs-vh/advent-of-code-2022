use crate::solution::Solution;

mod day01;

pub fn get_solution(day: &u8) -> Option<impl Solution> {
    match day {
        1 => Some(day01::CalorieCouting),
        _ => None,
    }
}
