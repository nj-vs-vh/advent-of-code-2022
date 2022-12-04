use crate::utils::read_input;

pub fn camp_cleanup() {
    let input = read_input(4, false);

    struct Range(u32, u32);

    impl Range {
        fn contains(&self, other: &Range) -> bool {
            self.0 <= other.0 && self.1 >= other.1
        }

        fn overlaps(&self, other: &Range) -> bool {
            !(self.1 < other.0 || self.0 > other.1)
        }
    }

    fn parse_range(s: &str) -> Range {
        let mut it = s.split("-").map(|d| d.parse::<u32>().unwrap());
        Range(it.next().unwrap(), it.next().unwrap())
    }

    fn parse_ranges(line: &str) -> (Range, Range) {
        let mut it = line.split(",").map(parse_range);
        (it.next().unwrap(), it.next().unwrap())
    }

    let range_iter: Vec<(Range, Range)> = input.lines().map(parse_ranges).collect();

    println!(
        "pt1: {}",
        range_iter
            .iter()
            .map(|(r1, r2)| (r1.contains(&r2) || r2.contains(&r1)) as u32)
            .sum::<u32>()
    );

    println!(
        "pt2: {}",
        range_iter
            .iter()
            .map(|(r1, r2)| (r1.overlaps(&r2)) as u32)
            .sum::<u32>()
    );
}
