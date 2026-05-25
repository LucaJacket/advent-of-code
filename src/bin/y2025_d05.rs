//!
//! Approach:
//! - split into ingredients and ranges, then parse every range
//! - for each ingredient, check if any range contains the ingredient
//!
//! Part 2:
//! - sort the ranges by start
//! - create a new Vec
//! - for each range:
//!   if it overlaps with the last range, extend the last range
//!   else add the range
//!

use advent_of_code::common::read_input;
use std::ops::RangeInclusive;

fn main() {
    let input = read_input(2025, 5);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_range(range: &str) -> RangeInclusive<u64> {
    let (start, end) = range.split_once('-').unwrap();
    let start = start.parse::<u64>().unwrap();
    let end = end.parse::<u64>().unwrap();
    start..=end
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_unstable_by_key(|range| *range.start());

    ranges.into_iter().fold(Vec::new(), |mut merged, range| {
        match merged.last_mut() {
            Some(current) if current.contains(range.start()) => {
                let end = (*current.end()).max(*range.end());
                *current = (*current.start())..=end
            }
            _ => merged.push(range),
        }
        merged
    })
}

fn part1(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = ranges.lines().map(parse_range).collect();
    ingredients
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count()
}

fn part2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<u64>> = ranges.lines().map(parse_range).collect();
    merge_ranges(ranges)
        .into_iter()
        .map(RangeInclusive::count)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = "3-5
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
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
