//!
//! Approach:
//! - one-hot encode the 1st level (S = 1)
//! - for each level, compute next state and occurred splittings:
//!   if current[i] contains no beams, skip
//!   if found a splitter, increment counter and split
//!   else go through
//! - return total splittings
//!
//! Part 2:
//! - return sum final beams
//!

use advent_of_code::common::read_input;

fn main() {
    let input = read_input(2025, 7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const START: char = 'S';
const SPLITTER: char = '^';
const EMPTY: char = '.';

fn step(current: &[usize], splitters: &[char]) -> (Vec<usize>, usize) {
    let n = current.len();

    let mut next = vec![0; n];
    let mut splittings = 0;

    for i in 0..n {
        let beams = current[i];
        if beams == 0 {
            continue;
        }
        match splitters[i] {
            SPLITTER => {
                splittings += 1;
                if i > 0 {
                    next[i - 1] += beams;
                }
                if i + 1 < n {
                    next[i + 1] += beams;
                }
            }
            EMPTY => next[i] += beams,
            _ => unreachable!(),
        }
    }

    (next, splittings)
}

fn part1(input: &str) -> usize {
    let levels = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = levels[0]
        .iter()
        .map(|&x| (x == START) as usize)
        .collect::<Vec<_>>();

    levels[1..]
        .iter()
        .scan(start, |current, splitters| {
            let (next, splittings) = step(current, splitters);
            *current = next;
            Some(splittings)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let levels = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = levels[0]
        .iter()
        .map(|&x| (x == START) as usize)
        .collect::<Vec<_>>();

    levels[1..]
        .iter()
        .fold(start, |current, splitters| {
            let (next, _) = step(&current, splitters);
            next
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
