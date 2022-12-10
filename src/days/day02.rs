use crate::utils::read_input;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Shape {
    fn match_with(&self, other: &Shape) -> Outcome {
        if self == other {
            return Outcome::Draw;
        } else if &self.get_winning_shape() == other {
            return Outcome::Lose;
        } else {
            return Outcome::Win;
        }
    }

    fn get_winning_shape(&self) -> Shape {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn get_losing_shape(&self) -> Shape {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn score(&self) -> u32 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn calculate_round_score_pt1(line: &str) -> u32 {
    let mut chars_iter = line.split(" ");

    use Shape::*;
    let opponent_shape = match chars_iter.next().unwrap() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => {
            panic!("opponent move must be A, B or C");
        }
    };
    let my_shape = match chars_iter.next().unwrap() {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => {
            panic!("my move must be X, Y or Z");
        }
    };
    return my_shape.score() + my_shape.match_with(&opponent_shape) as u32;
}
fn calculate_round_score_pt2(line: &str) -> u32 {
    let mut chars_iter = line.split(" ");

    use Shape::*;
    let opponent_shape = match chars_iter.next().unwrap() {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => {
            panic!("opponent move must be A, B or C");
        }
    };
    let my_shape = match chars_iter.next().unwrap() {
        "X" => opponent_shape.get_losing_shape(),
        "Y" => opponent_shape,
        "Z" => opponent_shape.get_winning_shape(),
        _ => {
            panic!("my move must be X, Y or Z");
        }
    };
    return my_shape.score() + my_shape.match_with(&opponent_shape) as u32;
}
pub fn rock_paper_scissors() {
    let input = read_input(2, false);
    println!(
        "pt1: {}",
        input
            .split("\n")
            .map(calculate_round_score_pt1)
            .sum::<u32>()
    );
    println!(
        "pt2: {}",
        input
            .split("\n")
            .map(calculate_round_score_pt2)
            .sum::<u32>()
    );
}
