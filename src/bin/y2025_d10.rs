//
// Approach:
// - parse every diagram and button to Vec<bool>
// - BFS search the solutions tree:
//   at each level, press 1 button and check if target is reached
// - Pruning:
//   pressing twice a button cancels its effect
//   order of pressing is irrelevant
//
// Part 2:
// - parse every requirement to Vec<usize>
// - compute steps:
//   find all possible combinations of buttons which match the parities of the requirements
//
//

use advent_of_code::common::read_input;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::from_fn;

fn main() {
    let input = read_input(2025, 10);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

type Diagram = Vec<bool>;
type Button = Vec<bool>;
type Requirements = Vec<usize>;
type LightSolution = Vec<bool>;
type JoltageSolution = Vec<usize>;

fn parse_machine(machine: &str) -> (Diagram, Vec<Button>, Requirements) {
    let mut parts = machine.split_whitespace();
    let diagram: Diagram = parts
        .next()
        .map(|diagram_str| {
            diagram_str
                .trim_start_matches('[')
                .trim_end_matches(']')
                .as_bytes()
                .iter()
                .map(|&ch| match ch {
                    b'#' => true,
                    b'.' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .expect("no diagram");
    let requirements: Requirements = parts
        .next_back()
        .map(|requirements_str| {
            requirements_str
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|num| num.parse().expect("failed to parse requirements"))
                .collect()
        })
        .expect("no requirements");
    let buttons: Vec<Button> = parts
        .map(|button_str| {
            button_str
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|num| num.parse().expect("failed to parse button"))
                .fold(
                    vec![false; diagram.len()],
                    |mut button: Button, i: usize| {
                        button[i] = true;
                        button
                    },
                )
        })
        .collect();

    (diagram, buttons, requirements)
}

fn configure_lights(diagram: Diagram, buttons: &[Button]) -> impl Iterator<Item = LightSolution> {
    let mut seen: HashSet<LightSolution> = HashSet::new();
    let mut queue: VecDeque<LightSolution> = VecDeque::new();

    let start: LightSolution = vec![false; buttons.len()];
    queue.push_back(start);

    from_fn(move || {
        while let Some(current) = queue.pop_front() {
            if !seen.insert(current.clone()) {
                continue;
            }

            (0..buttons.len()).filter(|&i| !current[i]).for_each(|i| {
                let mut next: LightSolution = current.clone();
                next[i] = true;
                queue.push_back(next);
            });

            let output: Diagram = (0..buttons.len()).filter(|&i| current[i]).fold(
                vec![false; diagram.len()],
                |output, i| {
                    output
                        .iter()
                        .zip(buttons[i].iter())
                        .map(|(&x, &y)| x ^ y)
                        .collect()
                },
            );
            if output == diagram {
                return Some(current);
            }
        }

        None
    })
}

fn configure_joltages(
    requirements: Requirements,
    buttons: &[Button],
    memo: &mut HashMap<Requirements, Option<JoltageSolution>>,
    light_memo: &mut HashMap<Diagram, Vec<LightSolution>>,
) -> Option<JoltageSolution> {
    if let Some(cached) = memo.get(&requirements) {
        return cached.clone();
    }

    if requirements.iter().all(|&requirement| requirement == 0) {
        let end: JoltageSolution = vec![0; buttons.len()];
        return Some(end);
    }

    let parities: Diagram = requirements
        .iter()
        .map(|&requirement| !requirement.is_multiple_of(2))
        .collect();
    let light_solutions = light_memo
        .entry(parities.clone())
        .or_insert_with(|| configure_lights(parities.clone(), buttons).collect())
        .clone();
    let mut best: Option<JoltageSolution> = None;
    'outer: for light_solution in light_solutions.iter() {
        let mut remaining: Requirements = requirements.clone();
        for i in 0..light_solution.len() {
            if light_solution[i] {
                for j in 0..requirements.len() {
                    if buttons[i][j] {
                        if remaining[j] == 0 {
                            continue 'outer;
                        }
                        remaining[j] -= 1;
                    }
                }
            }
        }
        for i in 0..remaining.len() {
            remaining[i] /= 2;
        }

        // check if there is solution
        if let Some(partial) = configure_joltages(remaining, buttons, memo, light_memo) {
            let current: JoltageSolution = (0..buttons.len())
                .map(|i| light_solution[i] as usize + 2 * partial[i])
                .collect();

            let current_cost: usize = current.iter().sum();

            match &best {
                None => best = Some(current),
                Some(best_sol) => {
                    let best_cost: usize = best_sol.iter().sum();
                    if current_cost < best_cost {
                        best = Some(current);
                    }
                }
            }
        }
    }

    memo.insert(requirements.clone(), best.clone());
    best
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_machine)
        .map(|(diagram, buttons, _)| {
            configure_lights(diagram, &buttons)
                .next()
                .map(|solution| solution.into_iter().filter(|&pressed| pressed).count())
                .expect("no solution")
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_machine)
        .map(|(_, buttons, requirements)| {
            configure_joltages(
                requirements,
                &buttons,
                &mut HashMap::new(),
                &mut HashMap::new(),
            )
            .map(|solution| solution.into_iter().sum::<usize>())
            .expect("no solution")
        })
        .inspect(|joltage| println!("{}", joltage))
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
