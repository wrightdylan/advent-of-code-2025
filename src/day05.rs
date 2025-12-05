use btree_range_map::{AsRange, RangeSet};
use std::ops::Bound::Included;

type Fresh = RangeSet<usize>;
type IDs = Vec<usize>;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Fresh, IDs) {
    let (input_range, input_available) = input.split_once("\n\n").unwrap();

    (
        input_range
            .lines()
            .filter_map(|line| {
                let (start, end) = line.split_once('-').unwrap();
                Some(start.parse().unwrap()..=end.parse().unwrap())
            })
            .collect(),
        input_available
            .lines()
            .map(|line| line.parse().unwrap())
            .collect()
    )
}

#[aoc(day5, part1)]
pub fn solve_part1((fresh, available): &(Fresh, IDs)) -> usize {
    let mut sum = 0;

    for &item in available {
        if fresh.contains(item) {
            sum += 1;
        }
    }

    sum
}

#[aoc(day5, part2)]
pub fn solve_part2((fresh, _): &(Fresh, IDs)) -> usize {
    let mut sum = 0;

    for range in fresh {
        sum += match (range.start(), range.end()) {
            (Included(start_val), Included(end_val)) => (*start_val..=*end_val).count(),
            _ => panic!("Expected included start and end bounds!"),
        };
    }
    
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 14);
    }
}