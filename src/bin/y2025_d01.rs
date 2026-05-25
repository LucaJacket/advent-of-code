//!
//! Approach:
//! - parse every rotation (signed integer: left = negative, right = positive)
//! - apply rotation
//! - detect if dial is on 0 (modular arithmetic)
//! - count
//!
//! Part 2:
//! - repeat n times a +1 or -1 rotation
//!

use advent_of_code::common::read_input;
use std::iter::repeat_n;

fn main() {
    let input = read_input(2025, 1);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const START: i32 = 50;
const SIZE: i32 = 100;

fn parse_rotation(rotation: &str) -> i32 {
    let mut chars = rotation.chars();
    let direction = match chars.next().unwrap() {
        'L' => -1,
        'R' => 1,
        _ => unreachable!(),
    };
    let distance = chars.as_str().parse::<i32>().unwrap();
    direction * distance
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_rotation)
        .scan(START, |position, rotation| {
            *position = (*position + rotation).rem_euclid(SIZE);
            Some(*position)
        })
        .filter(|&position| position == 0)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_rotation)
        .flat_map(|rotation| repeat_n(rotation.signum(), rotation.unsigned_abs() as usize))
        .scan(START, |position, rotation| {
            *position = (*position + rotation).rem_euclid(SIZE);
            Some(*position)
        })
        .filter(|&position| position == 0)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
