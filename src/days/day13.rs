use std::{cmp::Ordering, fmt::Display, str::Chars};

use itertools::Itertools;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Int(u32),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i)?,
            Value::List(items) => {
                write!(f, "[")?;
                for (idx, item) in items.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if idx != items.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
            }
        };
        Ok(())
    }
}

impl Value {
    fn parse(s: &str) -> Value {
        // inner recursive function, consumes value to the end of the string / to the sibling list element
        enum ParsingEndReason {
            LineEnd,
            Sibling,
            ListEnd,
        }

        fn parse_from_chars(chars: &mut Chars) -> (Option<Value>, ParsingEndReason) {
            let first = chars.next().unwrap();
            match first {
                '[' => {
                    let mut list: Vec<Value> = Vec::new();
                    loop {
                        let (maybe_item, item_parsing_end_reason) = parse_from_chars(chars);
                        if let Some(item) = maybe_item {
                            list.push(item);
                        }
                        match item_parsing_end_reason {
                            ParsingEndReason::LineEnd => {
                                break;
                            }
                            ParsingEndReason::Sibling => {}
                            ParsingEndReason::ListEnd => {
                                break;
                            }
                        }
                    }
                    let mut end_reason = ParsingEndReason::LineEnd;
                    while let Some(ch) = chars.next() {
                        if ch == ',' {
                            end_reason = ParsingEndReason::Sibling;
                            break;
                        } else if ch == ']' {
                            end_reason = ParsingEndReason::ListEnd;
                            break;
                        }
                    }

                    (Some(Value::List(list)), end_reason)
                }
                ']' => (None, ParsingEndReason::ListEnd),
                _ => {
                    let mut int_str = String::new();
                    int_str.push(first);
                    let mut end_reason = ParsingEndReason::LineEnd;
                    while let Some(ch) = chars.next() {
                        if ch.is_digit(10) {
                            int_str.push(ch);
                        } else if ch == ',' {
                            end_reason = ParsingEndReason::Sibling;
                            break;
                        } else if ch == ']' {
                            end_reason = ParsingEndReason::ListEnd;
                            break;
                        }
                    }
                    // println!("{}", int_str);
                    (Some(Value::Int(int_str.parse().unwrap())), end_reason)
                }
            }
        }

        parse_from_chars(&mut s.chars()).0.unwrap()
    }
}

fn is_ordered_correctly(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Int(int_left), Value::Int(int_right)) => {
            if int_left == int_right {
                None
            } else {
                Some(int_left < int_right)
            }
        }
        (Value::List(items_left), Value::List(items_right)) => {
            let mut iter_left = items_left.iter();
            let mut iter_right = items_right.iter();
            loop {
                let maybe_item_left = iter_left.next();
                let maybe_item_right = iter_right.next();
                match (maybe_item_left, maybe_item_right) {
                    (None, None) => {
                        return None;
                    }
                    (None, Some(_v)) => {
                        return Some(true);
                    }
                    (Some(_v), None) => {
                        return Some(false);
                    }
                    (Some(item_left), Some(item_right)) => {
                        if let Some(items_ordered_correctly) =
                            is_ordered_correctly(item_left, item_right)
                        {
                            return Some(items_ordered_correctly);
                        }
                    }
                }
            }
        }
        (list_value, Value::Int(i)) => {
            is_ordered_correctly(list_value, &Value::List(Vec::from([Value::Int(*i)])))
        }
        (Value::Int(i), list_value) => {
            is_ordered_correctly(&Value::List(Vec::from([Value::Int(*i)])), list_value)
        }
    }
}

pub struct DistressSignal;

impl Solution for DistressSignal {
    type InputT = Vec<(Value, Value)>;
    type OutputT = usize;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut res: Vec<(Value, Value)> = Vec::new();
        for input_p in input_raw.split("\n\n") {
            // println!("\n{input_p}");
            let mut lines_iter = input_p.lines();
            let left = Value::parse(lines_iter.next().unwrap());
            // println!("{left}");
            let right = Value::parse(lines_iter.next().unwrap());
            // println!("{right}");
            res.push((left, right));
        }
        res
    }

    fn solve_pt1(
        &self,
        input: Self::InputT,
        _visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        input
            .iter()
            .enumerate()
            .map(|(idx, (l, r))| {
                if is_ordered_correctly(l, r).unwrap() {
                    (idx + 1) as usize
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_pt2(
        &self,
        input: Self::InputT,
        _visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let divider_1 = Value::List(Vec::from([Value::List(Vec::from([Value::Int(2)]))]));
        let divider_2 = Value::List(Vec::from([Value::List(Vec::from([Value::Int(6)]))]));

        let mut packets: Vec<Value> = input
            .into_iter()
            .flat_map(|pair| Vec::from([pair.0, pair.1]))
            .collect();
        packets.push(divider_1.clone());
        packets.push(divider_2.clone());
        packets.sort_by(|a, b| {
            if let Some(is_ordered_correctly_) = is_ordered_correctly(a, b) {
                if is_ordered_correctly_ {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        });

        let (divied_1_idx, _) = packets.iter().find_position(|p| **p == divider_1).unwrap();
        let (divied_2_idx, _) = packets.iter().find_position(|p| **p == divider_2).unwrap();
        (divied_1_idx + 1) * (divied_2_idx + 1)
    }
}
