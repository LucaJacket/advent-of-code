//
// Approach:
// - parse every range
// - cast the number into digits (String)
// - divide the digits in 2 blocks and check if they are equal
//
// Part 2:
// - just check every possible block size: from 2 to id.len()
//

use advent_of_code::common::read_input;
use std::ops::RangeInclusive;

fn main() {
    let input = read_input(2025, 2);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_range(range: &str) -> RangeInclusive<u64> {
    let (start, end) = range.split_once('-').expect("failed to parse range");
    let start = start.parse().expect("failed to parse start");
    let end = end.parse().expect("failed to parse end");
    start..=end
}

fn is_repeated_block(id: &str, num_blocks: usize) -> bool {
    if !id.len().is_multiple_of(num_blocks) {
        return false;
    }
    let block_size = id.len() / num_blocks;
    (block_size..id.len())
        .step_by(block_size)
        .all(|i| id[0..block_size] == id[i..i + block_size])
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
            (2..=digits.len()).any(|num_blocks| is_repeated_block(&digits, num_blocks))
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
