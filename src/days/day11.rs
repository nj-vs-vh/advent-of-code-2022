use std::collections::VecDeque;

use regex::Regex;

use crate::{solution::Solution, utils::repeated_char, visualizer::Visualizer};

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

impl Operation {
    fn from_operation_rhs(s: &str) -> Operation {
        if s == "old * old" {
            return Operation::Square;
        } else {
            let op_num = s.strip_prefix("old ").unwrap();
            match op_num.chars().next().unwrap() {
                '*' => Operation::Mult(op_num[2..].parse().expect("")),
                '+' => Operation::Add(op_num[2..].parse().expect("")),
                _ => {
                    panic!("Can't parse operation from RHS: {}", s);
                }
            }
        }
    }

    fn perform(&self, inp: &u64) -> u64 {
        match self {
            Operation::Add(n) => *inp + n,
            Operation::Mult(n) => *inp * n,
            Operation::Square => *inp * *inp,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test_divisible_by: u64,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

fn visualize_monkeys(
    vis: &mut dyn Visualizer,
    round: usize,
    monkeys: &Vec<Monkey>,
    inspected: Option<u64>,
    op: Option<&Operation>,
) {
    if !vis.is_enabled() {
        return;
    }
    vis.write_line(&format!("round: {}", round + 1));
    if let Some(inspected_worry) = inspected {
        vis.write_str(&format!("ispected: {: >12}", inspected_worry));
        if let Some(operation) = op {
            vis.write_line(&format!(" <- {:?}", operation));
        } else {
            vis.write_newline();
        }
    } else {
        vis.write_char('\n');
    }
    for m in monkeys {
        vis.write_str(&format!("[{: ^10}]", m.idx));
    }
    vis.write_newline();
    let mut item_idx = 0;
    loop {
        let mut has_printed = false;
        for m in monkeys {
            if m.items.len() > item_idx {
                vis.write_str(&format!("{: ^12}", m.items[item_idx]));
                has_printed = true;
            } else {
                vis.write_str(&repeated_char(' ', 12));
            }
        }
        vis.write_newline();
        item_idx += 1;
        if !has_printed {
            break;
        }
    }
    vis.end_frame();
}

pub struct MonkeyInTheMiddle;

impl Solution for MonkeyInTheMiddle {
    type InputT = Vec<Monkey>;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut monkeys: Vec<Monkey> = Vec::new();

        let monkey_num_re = Regex::new(r"Monkey (\d+):").expect("");
        let starting_items_re = Regex::new(r"\s*Starting items: (.+)").expect("");
        let operation_re = Regex::new(r"\s*Operation: new = (.+)").expect("");
        let test_divisible_by_re = Regex::new(r"\s*Test: divisible by (.+)").expect("");
        let if_true_throw_to_re = Regex::new(r"\s*If true: throw to monkey (.+)").expect("");
        let if_false_throw_to_re = Regex::new(r"\s*If false: throw to monkey (.+)").expect("");
        for input_block in input_raw.split("\n\n") {
            let mut monkey_lines = input_block.lines();
            let monkey_index: usize = monkey_num_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1]
                .parse()
                .expect("");

            let starting_items_str = &starting_items_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1];
            let starting_items: VecDeque<u64> = starting_items_str
                .split(", ")
                .map(|i| i.parse().expect(""))
                .collect();

            let operation_rhs = &operation_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1];
            let operation = Operation::from_operation_rhs(&operation_rhs);

            let test_divisible_by: u64 = test_divisible_by_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1]
                .parse()
                .expect("");

            let if_true_throw_to: usize = if_true_throw_to_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1]
                .parse()
                .expect("");

            let if_false_throw_to: usize = if_false_throw_to_re
                .captures_iter(monkey_lines.next().unwrap())
                .next()
                .unwrap()[1]
                .parse()
                .expect("");

            monkeys.push(Monkey {
                idx: monkey_index,
                items: starting_items,
                operation,
                test_divisible_by,
                if_true_throw_to,
                if_false_throw_to,
            })
        }
        monkeys
    }

    fn solve_pt1(
        &self,
        mut monkeys: Self::InputT,
        vis: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        visualize_monkeys(vis, 0, &monkeys, None, None);

        let mut monkey_business: Vec<u64> = vec![0; monkeys.len()];

        for round in 0..20 {
            for m_id in 0..monkeys.len() {
                while let Some(worry_level) = monkeys[m_id].items.pop_front() {
                    monkey_business[m_id] += 1;
                    visualize_monkeys(
                        vis,
                        round,
                        &monkeys,
                        Some(worry_level),
                        Some(&monkeys[m_id].operation),
                    );
                    let worry_level_testing = monkeys[m_id].operation.perform(&worry_level) / 3;
                    visualize_monkeys(vis, round, &monkeys, Some(worry_level_testing), None);
                    let throw_to = if worry_level_testing % monkeys[m_id].test_divisible_by == 0 {
                        monkeys[m_id].if_true_throw_to
                    } else {
                        monkeys[m_id].if_false_throw_to
                    };
                    monkeys[throw_to].items.push_back(worry_level_testing);
                }
            }
        }

        monkey_business.sort();
        monkey_business.reverse();
        monkey_business[0..2].iter().product()
    }

    fn solve_pt2(
        &self,
        mut monkeys: Self::InputT,
        vis: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let mut monkey_business: Vec<u64> = vec![0; monkeys.len()];
        let magic_constant: u64 = monkeys.iter().map(|m| m.test_divisible_by).product();
        println!("Magic constant: {}", magic_constant);

        visualize_monkeys(vis, 0, &monkeys, None, None);
        for round in 0..10000 {
            for m_id in 0..monkeys.len() {
                while let Some(worry_level) = monkeys[m_id].items.pop_front() {
                    monkey_business[m_id] += 1;
                    visualize_monkeys(
                        vis,
                        round,
                        &monkeys,
                        Some(worry_level),
                        Some(&monkeys[m_id].operation),
                    );
                    let worry_level_testing =
                        monkeys[m_id].operation.perform(&worry_level) % magic_constant;
                    visualize_monkeys(vis, round, &monkeys, Some(worry_level_testing), None);
                    let throw_to = if worry_level_testing % monkeys[m_id].test_divisible_by == 0 {
                        monkeys[m_id].if_true_throw_to
                    } else {
                        monkeys[m_id].if_false_throw_to
                    };
                    monkeys[throw_to].items.push_back(worry_level_testing);
                }
            }
        }

        monkey_business.sort();
        monkey_business.reverse();
        monkey_business[0..2].iter().product()
    }
}
