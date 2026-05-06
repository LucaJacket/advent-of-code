//
// Approach:
// - split into ingredients and ranges, then parse every range
// - for each ingredient, check if any range contains the ingredient
//
// Part 2:
// - sort the ranges by start
// - create a new Vec
// - set the 1st range as the current range
// - for each next range:
//   if it overlaps with the current range, extend the current range
//   else push the current range into the Vec and set the next range as the current range
// - finally, push the current range into the Vec
//

use std::iter::from_fn;
use advent_of_code::common::read_input;
use std::ops::RangeInclusive;

fn main() {
    let input = read_input(2025, 5);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_range(range: &str) -> RangeInclusive<u64> {
    let (start, end) = range.split_once('-').expect("range");
    let start = start.parse().expect("start");
    let end = end.parse().expect("end");
    start..=end
}

fn merge_ranges(ranges: impl IntoIterator<Item = RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut sorted = ranges.into_iter().collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|range| *range.start());
    let mut sorted = sorted.into_iter();
    
    from_fn(move || {
        let mut current = sorted.next().expect("range");
        
    });
    
    sorted.into_iter().fold(Vec::new(), |merged, range| {
        
    });
    let mut sorted = sorted.into_iter();

    let mut merged = Vec::new();
    if let Some(first) = sorted.next() {
        let mut current = first;
        for next in sorted {
            if current.contains(next.start()) {
                current = (*current.start())..=(*current.end()).max(*next.end());
            } else {
                merged.push(current);
                current = next;
            }
        }
        merged.push(current)
    }
    merged
}

fn part1(input: &str) -> usize {
    let (ranges, ingredients) = input
        .split_once("\n\n")
        .expect("failed to split range and ingredients");
    let ranges: Vec<_> = ranges.lines().map(parse_range).collect();
    ingredients
        .lines()
        .map(|line| line.parse().expect("failed to parse ingredient"))
        .filter(|&ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count()
}

fn part2(input: &str) -> usize {
    let (ranges, _) = input
        .split_once("\n\n")
        .expect("failed to split range and ingredients");
    merge_ranges(ranges.lines().map(parse_range))
        .into_iter()
        .map(RangeInclusive::count)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

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
