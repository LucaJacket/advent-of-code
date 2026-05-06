//
// Approach:
// - parse every bank to byte slice &[u8]
// - extract 1st battery (max. power) among all but the last
// - extract 2nd battery among all following the 1st
//
// Part 2:
// - just repeat 12 times, each time excluding (suppose the i-th battery is picked at index idx):
//   the first idx + 1 batteries and the last 12 - i
//

use advent_of_code::common::{parse_from_digits, read_input};
use std::cmp::Reverse;

fn main() {
    let input = read_input(2025, 3);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn extract_total_power(bank: &[u8], n: usize) -> u64 {
    let digits = (1..=n).scan(0, |start, i| {
        let end = bank.len() - (n - i);
        let (&power, idx) = bank[*start..end]
            .iter()
            .zip(*start..)
            .max_by_key(|&(&power, idx)| (power, Reverse(idx)))
            .unwrap();
        *start = idx + 1;
        Some(power)
    });
    parse_from_digits(digits)
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|bank| extract_total_power(bank, 2))
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|bank| extract_total_power(bank, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}
