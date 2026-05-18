//
// Approach:
// - parse every diagram and button to Vec<bool>
// - BFS search:
//   at each choice, press button and check if target is reached
// - Pruning:
//   pressing twice a button cancels its effect
//   order of pressing is irrelevant
//
// Part 2:
// - parse every requirement to Vec<usize>
// - DFS search:
//   find all possible combinations of buttons which match the parities of the requirements
//   for each of those, compute remaining requirements, halve and recurse
//   at least one of those combinations will lead to a possible solution
//

use advent_of_code::common::read_input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::from_fn;

fn main() {
    let input = read_input(2025, 10);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse_machine(machine: &str) -> (Vec<bool>, Vec<Vec<bool>>, Vec<usize>) {
    let mut parts = machine.split_whitespace();
    let diagram = parts
        .next()
        .map(|diagram| {
            diagram
                .trim_start_matches('[')
                .trim_end_matches(']')
                .as_bytes()
                .iter()
                .map(|&ch| match ch {
                    b'#' => true,
                    b'.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap();
    let requirements = parts
        .next_back()
        .map(|requirements| {
            requirements
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap();
    let buttons = parts
        .map(|button| {
            button
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .fold(vec![false; diagram.len()], |mut button, i| {
                    button[i] = true;
                    button
                })
        })
        .collect::<Vec<_>>();

    (diagram, buttons, requirements)
}

fn configure_lights(diagram: &[bool], buttons: &[Vec<bool>]) -> impl Iterator<Item = Vec<bool>> {
    let n = diagram.len();
    let m = buttons.len();

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let start = vec![false; m];
    let output = vec![false; n];
    queue.push_back((start, output));

    from_fn(move || {
        while let Some((current, output)) = queue.pop_front() {
            if !seen.insert(current.clone()) {
                continue;
            }

            for i in 0..m {
                if current[i] {
                    continue;
                }

                let mut next = current.clone();
                next[i] = true;
                let next_output = output
                    .iter()
                    .zip(buttons[i].iter())
                    .map(|(&x, &y)| x ^ y)
                    .collect::<Vec<_>>();

                queue.push_back((next, next_output));
            }

            if output == diagram {
                return Some(current);
            }
        }

        None
    })
}

fn configure_joltages(requirements: &[usize], buttons: &[Vec<bool>]) -> usize {
    fn dfs(
        remaining: Vec<usize>,
        buttons: &[Vec<bool>],
        solution_memo: &mut HashMap<Vec<usize>, Option<usize>>,
        parity_memo: &mut HashMap<Vec<bool>, Vec<Vec<bool>>>,
    ) -> Option<usize> {
        let n = remaining.len();
        let m = buttons.len();

        if let Some(&solution) = solution_memo.get(&remaining) {
            return solution;
        }

        if remaining.iter().all(|&x| x == 0) {
            return Some(0);
        }

        let parity = remaining
            .iter()
            .map(|&x| !x.is_multiple_of(2))
            .collect::<Vec<_>>();

        let parity_solutions = parity_memo
            .entry(parity.clone())
            .or_insert_with(|| configure_lights(&parity, buttons).collect::<Vec<_>>())
            .clone();

        let mut best = None;
        'outer: for parity in &parity_solutions {
            let mut next_remaining = remaining.to_vec();
            for i in 0..m {
                if !parity[i] {
                    continue;
                }

                for j in 0..n {
                    if buttons[i][j] {
                        if next_remaining[j] == 0 {
                            continue 'outer;
                        }

                        next_remaining[j] -= 1;
                    }
                }
            }

            for x in &mut next_remaining {
                *x /= 2;
            }

            if let Some(partial) = dfs(next_remaining, buttons, solution_memo, parity_memo) {
                let candidate = parity.iter().filter(|&&pressed| pressed).count() + 2 * partial;
                if best.is_none() || candidate < best.unwrap() {
                    best = Some(candidate);
                }
            }
        }

        solution_memo.insert(remaining.to_vec(), best);

        best
    }

    dfs(
        requirements.to_vec(),
        buttons,
        &mut HashMap::new(),
        &mut HashMap::new(),
    )
    .unwrap()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_machine)
        .map(|(diagram, buttons, _)| {
            configure_lights(&diagram, &buttons)
                .next()
                .map(|solution| solution.into_iter().filter(|&pressed| pressed).count())
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_machine)
        .map(|(_, buttons, requirements)| configure_joltages(&requirements, &buttons))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), 33);
    }
}
