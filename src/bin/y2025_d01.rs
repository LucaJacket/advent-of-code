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
    let (direction, distance) = rotation.split_at(1);
    let distance = distance.parse::<i32>().unwrap();
    match direction {
        "L" => -distance,
        "R" => distance,
        _ => unreachable!(),
    }
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
    use crate::{part1, part2};

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
