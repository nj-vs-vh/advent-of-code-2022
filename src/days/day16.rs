use crate::solution::Solution;
use std::collections::{HashMap, HashSet};

use regex::Regex;

const MINUTE_COUNT: usize = 30;
// const MAX_VALVE_COUNT: usize = 10;
const MAX_VALVE_COUNT: usize = 60;

#[derive(Clone)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    neighbor_names: Vec<String>,
    id: usize, // set to sequential number, suitable for binary mask construction
}

type Valves = HashMap<String, Valve>;
type TravelTimes = HashMap<String, HashMap<String, u32>>;

fn calculate_travel_times(valves: &Valves) -> TravelTimes {
    valves
        .iter()
        .filter_map(|(n, v)| {
            if v.flow_rate > 0 || n == "AA" {
                Some(n.clone())
            } else {
                None
            }
        })
        .map(|start_valve_name| {
            // BFS to find (shortest travel times) from a given start valve
            let mut res: HashMap<String, u32> = HashMap::new();
            let mut visited: HashSet<String> = HashSet::new();
            let mut to_visit: Vec<&str> = vec![&start_valve_name];
            let mut step_counter = 0;
            while to_visit.len() > 0 {
                let mut to_visit_next: Vec<&str> = Vec::new();
                for valve_name in to_visit.into_iter() {
                    visited.insert(valve_name.to_owned());
                    let valve = &valves[valve_name];
                    if valve.name != start_valve_name && valve.flow_rate > 0 {
                        res.insert(valve_name.to_owned(), step_counter);
                    }
                    for v in valve
                        .neighbor_names
                        .iter()
                        .filter(|&n| !visited.contains(n))
                    {
                        to_visit_next.push(v);
                    }
                }
                to_visit = to_visit_next;
                step_counter += 1;
            }
            (start_valve_name.clone(), res)
        })
        .collect()
}

fn max_released_pressure(
    valves: &Valves,
    travel_times: &TravelTimes,
    opened_bitmask: u64,
    current_valve_name: &str,
    time_left: u32,
) -> u32 {
    return (&travel_times[current_valve_name])
        .iter()
        .filter_map(|(next_name, &travel_time)| {
            if travel_time + 1 > time_left {
                return None; // prune if the time is running out
            }

            let time_left_after_opening = time_left - travel_time - 1;
            let next = &valves[next_name];
            if (opened_bitmask & 1 << next.id) != 0 {
                return None; // prune if we've already visited it in this branch
            }

            return Some(
                next.flow_rate * time_left_after_opening
                    + max_released_pressure(
                        valves,
                        travel_times,
                        opened_bitmask | 1 << next.id,
                        next_name,
                        time_left_after_opening,
                    ),
            );
        })
        .max()
        .unwrap_or(0);
}

pub struct ProboscideaVolcanium;

impl<'a> Solution for ProboscideaVolcanium {
    type InputT = Valves;
    type OutputT = u32;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let mut res: Vec<Valve> = Vec::new();
        let valve_line_regex =
            Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w,\s]+)")
                .unwrap();
        for (idx, line) in input_raw.lines().enumerate() {
            let captures = valve_line_regex.captures(line).unwrap();
            let parts: Vec<&str> = captures
                .iter()
                .skip(1)
                .map(|maybe_match| maybe_match.unwrap().as_str())
                .collect();
            let name = parts[0];
            res.push(Valve {
                name: name.to_owned(),
                flow_rate: parts[1].parse().expect("Unable to parse flow rate"),
                neighbor_names: parts[2].split(", ").map(|s| s.to_owned()).collect(),
                id: idx,
            });
        }
        res.iter().map(|v| (v.name.clone(), v.clone())).collect()
    }

    fn solve_pt1(
        &self,
        input: Self::InputT,
        _visualizer: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let travel_times = calculate_travel_times(&input);
        return max_released_pressure(&input, &travel_times, 0, "AA", 30);
    }

    fn solve_pt2(
        &self,
        _input: Self::InputT,
        _v: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        return 0;
    }
}

// failed attempt at "smart DP solution"
impl ProboscideaVolcanium {
    #[allow(dead_code)]
    fn solve_pt1_failed(
        &self,
        input: Vec<Valve>,
        v: &mut dyn crate::visualizer::Visualizer,
    ) -> u32 {
        let valve_indices: HashMap<&str, usize> = HashMap::from_iter(
            input
                .iter()
                .enumerate()
                .map(|(idx, valve)| (valve.name.as_ref(), idx)),
        );
        // legend: gpr = guaranteed pressure release
        let mut max_gpr_and_open_valves =
            [[(0 as u32, [false; MAX_VALVE_COUNT]); MAX_VALVE_COUNT]; MINUTE_COUNT];
        let mut is_reachable = [[false; MAX_VALVE_COUNT]; MINUTE_COUNT];
        is_reachable[0][0] = true; // at t = 0 we're the first valve
        for minute in 0..MINUTE_COUNT - 1 {
            for (idx, valve) in input.iter().enumerate() {
                if !is_reachable[minute][idx] {
                    continue;
                }
                // we can do nothing at all
                is_reachable[minute + 1][idx] = true;
                let (max_gpr, open_valves) = max_gpr_and_open_valves[minute][idx];
                if max_gpr > max_gpr_and_open_valves[minute + 1][idx].0 {
                    max_gpr_and_open_valves[minute + 1][idx] = (max_gpr, open_valves);
                }
                // we can stay at this valve and open it (if not yet opened)
                if !max_gpr_and_open_valves[minute][idx].1[idx] && valve.flow_rate > 0 {
                    let max_gpr_if_open = max_gpr_and_open_valves[minute][idx].0
                        + valve.flow_rate * (MINUTE_COUNT - minute - 1) as u32;
                    if max_gpr_if_open > max_gpr_and_open_valves[minute + 1][idx].0 {
                        let mut new_open_mask = max_gpr_and_open_valves[minute][idx].1;
                        new_open_mask[idx] = true;
                        max_gpr_and_open_valves[minute + 1][idx] = (max_gpr_if_open, new_open_mask)
                    }
                }
                // or we can move to any of the neighbors
                for neighbor_name in valve.neighbor_names.iter() {
                    let neighbor_idx = *valve_indices.get(neighbor_name.as_str()).unwrap();
                    if max_gpr > max_gpr_and_open_valves[minute + 1][neighbor_idx].0 {
                        max_gpr_and_open_valves[minute + 1][neighbor_idx] = (max_gpr, open_valves);
                    }
                    is_reachable[minute + 1][neighbor_idx] = true;
                }
            }
            if v.is_enabled() {
                let next_minute = minute + 1;
                v.write_line(&format!("After minute {}\n", next_minute));
                for valve in input.iter() {
                    if valve.flow_rate > 0 {
                        v.write_str(&format!("{: ^6}", valve.name));
                    }
                }
                v.write_newline();
                for (idx, valve) in input.iter().enumerate() {
                    if valve.flow_rate == 0 {
                        continue;
                    }
                    if is_reachable[next_minute][idx] {
                        v.write_str(&format!("{: ^6}", "*"));
                    } else {
                        v.write_str(&format!("{: ^6}", " "));
                    }
                }
                v.write_newline();
                for (idx, valve) in input.iter().enumerate() {
                    if valve.flow_rate == 0 {
                        continue;
                    }
                    v.write_str(&format!(
                        "{: ^6}",
                        max_gpr_and_open_valves[next_minute][idx].0
                    ));
                }
                v.end_frame();
            }
        }
        return *max_gpr_and_open_valves[MINUTE_COUNT - 1]
            .iter()
            .map(|(max_gpr, _)| max_gpr)
            .max()
            .unwrap();
    }
}
