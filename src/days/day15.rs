use std::cmp::{max, min};

use crate::solution::Solution;

use pathfinder_geometry::vector::Vector2I;
use regex::Regex;

#[derive(Debug)]
pub struct Sensor {
    position: Vector2I,
    closest_beacon: Vector2I,
}

pub struct BeaconExclusionZone;

impl Solution for BeaconExclusionZone {
    type InputT = Vec<Sensor>;
    type OutputT = u64;

    fn parse_input(&self, input_raw: String) -> Self::InputT {
        let line_re = Regex::new(
            r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)",
        )
        .unwrap();
        input_raw
            .lines()
            .map(|l| {
                let captures = line_re.captures_iter(l).next().unwrap();
                Sensor {
                    position: Vector2I::new(
                        captures[1].parse::<i32>().unwrap(),
                        captures[2].parse::<i32>().unwrap(),
                    ),
                    closest_beacon: Vector2I::new(
                        captures[3].parse::<i32>().unwrap(),
                        captures[4].parse::<i32>().unwrap(),
                    ),
                }
            })
            .collect()
    }
    fn solve_pt1(
        &self,
        sensors: Self::InputT,
        v: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let y_scanning: i32 = if sensors.len() > 15 { 2000000 } else { 10 };

        let mut beaconless_ranges: Vec<(i32, i32)> = Vec::new();

        for (sensor_idx, sensor) in sensors.iter().enumerate() {
            v.write_line(&format!(
                "{}. at {:?}; closest beacon at {:?}",
                sensor_idx + 1,
                sensor.position,
                sensor.closest_beacon
            ));
            let delta = sensor.closest_beacon - sensor.position;
            let sensor_range = delta.x().abs() + delta.y().abs();
            let horiz_sensor_range = sensor_range - (sensor.position.y() - y_scanning).abs();
            if horiz_sensor_range >= 0 {
                let mut range_start = sensor.position.x() - horiz_sensor_range;
                let mut range_end = sensor.position.x() + horiz_sensor_range + 1;
                if sensor.closest_beacon.y() == y_scanning {
                    if sensor.closest_beacon.x() == range_start {
                        range_start += 1;
                    }
                    if sensor.closest_beacon.x() == range_end - 1 {
                        range_end -= 1;
                    }
                }
                if range_end > range_start {
                    beaconless_ranges.push((range_start, range_end));
                }
                v.write_line(&format!("Ranges raw: {:?}", beaconless_ranges));

                beaconless_ranges.sort_by(|a, b| (*a).0.cmp(&b.0));
                v.write_line(&format!("Ranges sorted: {:?}", beaconless_ranges));
                let mut i: usize = 0;
                while i < beaconless_ranges.len() - 1 {
                    if beaconless_ranges[i].1 < beaconless_ranges[i + 1].0 {
                        i += 1;
                    } else {
                        let first = beaconless_ranges[i];
                        let second = beaconless_ranges.remove(i + 1);
                        beaconless_ranges[i] = (min(first.0, second.0), max(first.1, second.1));
                    }
                }
                v.write_line(&format!("Ranges without overlaps: {:?}", beaconless_ranges));
            } else {
                v.write_line("Does not impact the line we're scanning");
            }
            v.end_frame();
        }
        v.write_line(&format!("Ranges final: {:?}", beaconless_ranges));

        beaconless_ranges.iter().map(|(f, b)| b - f).sum::<i32>() as u64
    }
    fn solve_pt2(
        &self,
        sensors: Self::InputT,
        _v: &mut dyn crate::visualizer::Visualizer,
    ) -> Self::OutputT {
        let max_coord = if sensors.len() > 15 { 4_000_000 } else { 20 };
        let y_range = (0, max_coord);
        let x_range = (0, max_coord);

        let mut beaconless_ranges: Vec<(i32, i32)> = Vec::with_capacity(30);
        for y_scanning in y_range.0..=y_range.1 {
            beaconless_ranges.clear();
            let mut new_beacon_found = true;
            for sensor in sensors.iter() {
                let delta = sensor.closest_beacon - sensor.position;
                let sensor_range = delta.x().abs() + delta.y().abs();
                let horiz_sensor_range = sensor_range - (sensor.position.y() - y_scanning).abs();
                if horiz_sensor_range >= 0 {
                    let range_start = sensor.position.x() - horiz_sensor_range;
                    let range_end = sensor.position.x() + horiz_sensor_range + 1;
                    if range_end > range_start {
                        beaconless_ranges.push((range_start, range_end));
                    }

                    beaconless_ranges.sort_by(|a, b| (*a).0.cmp(&b.0));

                    let mut i: usize = 0;
                    while i < beaconless_ranges.len() - 1 {
                        if beaconless_ranges[i].1 < beaconless_ranges[i + 1].0 {
                            i += 1;
                        } else {
                            let first = beaconless_ranges[i];
                            let second = beaconless_ranges.remove(i + 1);
                            beaconless_ranges[i] = (min(first.0, second.0), max(first.1, second.1));
                        }
                    }
                    if beaconless_ranges
                        .iter()
                        .any(|(start, end)| *start <= x_range.0 && *end > x_range.1)
                    {
                        new_beacon_found = false;
                        break;
                    }
                }
            }

            if new_beacon_found {
                let x_new_beacon = *beaconless_ranges
                    .iter()
                    .map(|(_, end)| end)
                    .filter(|&end| &x_range.0 <= end && end <= &x_range.1)
                    .next()
                    .unwrap();
                println!("{x_new_beacon}, {y_scanning}");
                return (x_new_beacon as u64) * 4_000_000 + (y_scanning as u64);
            }
        }
        0
    }
}
