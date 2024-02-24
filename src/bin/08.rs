advent_of_code::solution!(8);
use std::collections::VecDeque;

use advent_of_code::lcm;
use hashbrown::{HashMap, HashSet};

fn parse_nodes(node_input: &str) -> HashMap<&str, (&str, &str)> {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in node_input.lines() {
        nodes.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    nodes
}
// Reworked part 1 to be a special case of part 1 where we do BFS but the starting node is only AAA
pub fn part_one(input: &str) -> Option<usize> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let nodes = parse_nodes(nodes);

    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut step_counts: usize = 0;
    let start = "AAA";
    // There are 6 nodes ending with A and we're going through each of them
    // We're using BFS to search through the nodes until we end up in node ZZZ
    queue.push_back((start, 0));
    visited.insert(start);
    while let Some((current, steps)) = queue.pop_front() {
        if current == "ZZZ" {
            step_counts = steps;
            break;
        }
        let (left, right) = nodes.get(current).unwrap();
        if visited.insert(left) {
            queue.push_back((left, steps + 1));
        }
        if visited.insert(right) {
            queue.push_back((right, steps + 1));
        }
    }
    // Since we might take more than 1 loop through the set of instruction to find ZZZ,
    // We need to take the LCM of the number of steps it takes to get to ZZZ and
    // the number of instructions
    Some(lcm(&vec![instructions.len(), step_counts]))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, nodes) = input.split_once("\n\n").unwrap();
    let nodes = parse_nodes(nodes);

    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    // Vector to store the cycle lengths for each ghost to see Z node again
    let mut cycle_lengths: Vec<usize> = vec![];
    // There are 6 nodes ending with A and we're going through each of them
    // We're using BFS to search through the nodes until we end up in node ending with Z
    // Recording how many steps it took to get there
    for &start in nodes.keys().filter(|k| k.ends_with('A')) {
        queue.push_back((start, 0));
        visited.insert(start);
        while let Some((current, steps)) = queue.pop_front() {
            if current.ends_with('Z') {
                cycle_lengths.push(steps);
                break;
            }
            let (left, right) = nodes.get(current).unwrap();
            if visited.insert(left) {
                queue.push_back((left, steps + 1));
            }
            if visited.insert(right) {
                queue.push_back((right, steps + 1));
            }
        }
        queue.clear();
        visited.clear();
    }
    Some(lcm(&cycle_lengths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
