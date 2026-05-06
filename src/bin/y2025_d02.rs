//
// Approach:
// - parse every range and flatten
// - for each id: check if id is first block repeated twice
//
// Part 2:
// - check all possible values of block_count
//

use advent_of_code::common::read_input;
use std::ops::RangeInclusive;

fn main() {
    let input = read_input(2025, 2);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_range(range: &str) -> RangeInclusive<u64> {
    let (start, end) = range.split_once('-').unwrap();
    let start = start.parse::<u64>().unwrap();
    let end = end.parse::<u64>().unwrap();
    start..=end
}

fn is_repeated_block(id: &str, block_count: usize) -> bool {
    if !id.len().is_multiple_of(block_count) {
        return false;
    }
    let block = &id[..id.len() / block_count];
    id == block.repeat(block_count)
}

fn part1(input: &str) -> u64 {
    input
        .split(',')
        .flat_map(parse_range)
        .filter(|&id| {
            let digits = id.to_string();
            is_repeated_block(&digits, 2)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .split(',')
        .flat_map(parse_range)
        .filter(|&id| {
            let digits = id.to_string();
            (2..=digits.len()).any(|block_count| is_repeated_block(&digits, block_count))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
