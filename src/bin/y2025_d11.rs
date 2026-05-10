//
// Approach:
// - parse into direct acyclic graph (HashMap)
// - build topological order of nodes:
//   use in_degree to compute when to add node to the final order (number of nodes pointing = 0)
// - compute paths using dynamic programming:
//   start from (start, 1), visit neighbors and add the number of paths to reach them
//
// Part 2:
// - since the graph is acyclic, only one of the paths is possible:
//   svr -> fft -> dac -> out | svr -> dac -> fft -> out
// - the total number of paths can be obtained multiplying the number of paths in each subpath
//

use advent_of_code::common::read_input;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = read_input(2025, 11);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

trait Extension {
    fn topological_sort(&self) -> Vec<&str>;
    fn paths(&self, order: &[&str], start: &str, end: &str) -> usize;
}

impl Extension for HashMap<&str, Vec<&str>> {
    fn topological_sort(&self) -> Vec<&str> {
        let mut in_degree = HashMap::new();
        for (&device, outputs) in self.iter() {
            in_degree.entry(device).or_insert(0);
            for &output in outputs {
                *in_degree.entry(output).or_insert(0) += 1;
            }
        }

        let mut order = Vec::new();

        let mut queue = VecDeque::new();
        for (&device, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(device);
            }
        }
        while let Some(device) = queue.pop_front() {
            order.push(device);
            if let Some(outputs) = self.get(device) {
                for &output in outputs {
                    let degree = in_degree.get_mut(output).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(output);
                    }
                }
            }
        }

        order
    }

    fn paths(&self, order: &[&str], start: &str, end: &str) -> usize {
        let mut paths = HashMap::new();

        paths.insert(start, 1);
        order
            .iter()
            .skip_while(|&&device| device != start)
            .take_while(|&&device| device != end)
            .for_each(|&device| {
                if let Some(&path_count) = paths.get(device)
                    && let Some(outputs) = self.get(device)
                {
                    for &output in outputs {
                        *paths.entry(output).or_insert(0) += path_count;
                    }
                }
            });

        *paths.get(end).unwrap_or(&0)
    }
}

fn parse_device(device: &str) -> (&str, Vec<&str>) {
    let (device, outputs) = device.split_once(':').unwrap();
    let outputs = outputs.split_whitespace().collect::<Vec<_>>();

    (device, outputs)
}

fn part1(input: &str) -> usize {
    let graph = input.lines().map(parse_device).collect::<HashMap<_, _>>();
    let order = graph.topological_sort();

    graph.paths(&order, "you", "out")
}

fn part2(input: &str) -> usize {
    let graph = input.lines().map(parse_device).collect::<HashMap<_, _>>();
    let order = graph.topological_sort();

    let svr_fft = graph.paths(&order, "svr", "fft");
    let fft_dac = graph.paths(&order, "fft", "dac");
    let dac_out = graph.paths(&order, "dac", "out");
    let svr_dac = graph.paths(&order, "svr", "dac");
    let dac_fft = graph.paths(&order, "dac", "fft");
    let fft_out = graph.paths(&order, "fft", "out");

    svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const EXAMPLE1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2), 2);
    }
}
