use regex::Regex;

use crate::utils::read_input;

const DAY: u8 = 5;

type Stacks = Vec<Vec<char>>;

fn parse_stacks(s: &str) -> Stacks {
    let stacks_count = s.lines().last().unwrap().split("   ").count();
    let mut stacks: Stacks = (0..stacks_count).map(|_| Vec::new()).collect();
    let crate_regex = Regex::new(r"\[(\w)\]").unwrap();
    for line in s.lines().rev().skip(1) {
        for stack_i in 0..stacks_count {
            let maybe_crate = &line[stack_i * 4..((stack_i + 1) * 4) - 1];
            // println!("{}", maybe_crate);
            for crate_letter in crate_regex.captures_iter(maybe_crate) {
                for cg in crate_letter.iter().skip(1) {
                    if let Some(cg_match) = cg {
                        for char in cg_match.as_str().chars() {
                            stacks[stack_i].push(char)
                        }
                    }
                }
            }
        }
    }
    return stacks;
}

#[derive(Debug)]
struct MoveDef {
    move_count: usize,
    from: usize,
    to: usize,
}

impl MoveDef {
    fn parse(s: &str) -> MoveDef {
        let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let match_ = regex.captures_iter(s).next().unwrap();
        let mut move_def_parts_iter = match_.iter().skip(1);
        MoveDef {
            move_count: move_def_parts_iter
                .next()
                .unwrap()
                .unwrap()
                .as_str()
                .parse()
                .expect("move count must be a number"),
            from: move_def_parts_iter
                .next()
                .unwrap()
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("move count must be a number")
                - 1,
            to: move_def_parts_iter
                .next()
                .unwrap()
                .unwrap()
                .as_str()
                .parse::<usize>()
                .expect("move count must be a number")
                - 1,
        }
    }
}

fn to_answer(stacks: &Stacks) -> String {
    let mut res = String::new();
    for stack in stacks {
        res.push(stack.last().unwrap().to_owned());
    }
    return res;
}

pub fn supply_stack() {
    let input = read_input(DAY, false);
    let mut input_blocks = input.split("\n\n");
    let stacks_init_block = input_blocks.next().unwrap().to_owned();

    let move_definitions_block = input_blocks.next().unwrap().to_owned();
    let move_defs: Vec<MoveDef> = move_definitions_block
        .lines()
        .map(|l| MoveDef::parse(l))
        .collect();

    let mut stacks_pt1 = parse_stacks(&stacks_init_block);
    let mut stacks_pt2 = stacks_pt1.clone();

    for md in move_defs.iter() {
        for _ in 0..md.move_count {
            let crate_ = stacks_pt1[md.from].pop().unwrap();
            stacks_pt1[md.to].push(crate_);
        }
    }
    println!("pt1: {:?}", to_answer(&stacks_pt1));

    for md in move_defs.iter() {
        let mut picked_up: Vec<char> = Vec::new();
        for _ in 0..md.move_count {
            picked_up.push(stacks_pt2[md.from].pop().unwrap());
        }
        picked_up.reverse();
        for c in picked_up {
            stacks_pt2[md.to].push(c);
        }
    }
    println!("pt2: {:?}", to_answer(&stacks_pt2));
}
