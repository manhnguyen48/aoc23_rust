advent_of_code::solution!(5);
use advent_of_code::parse_unsigned;

// Parses the input string into an Input struct.
//
// Splits the input into chunks separated by double newlines.
// The first chunk contains the seeds, which are parsed as unsigned integers and converted to u64.
//
// The remaining chunks represent the maps. Each map is split into lines, skipping the first line with the map name.
// The remaining lines are parsed into unsigned integers representing the "to", "start", and "length" values for that map.
// convert them to (to, start, end) for easier usage later.
// These are collected into a Vec of [u64; 3] tuples for each map.
//
// Returns an Input struct containing the seeds and maps.
fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<[u64; 3]>>) {
    let chunks: Vec<_> = input.split("\n\n").collect();
    let seeds = parse_unsigned(chunks[0])
        .iter()
        .map(|&n| n as u64)
        .collect();

    let maps = chunks[1..]
        .iter()
        .map(|chunk| {
            chunk
                .lines()
                .skip(1) // Skipping the first line as it contains the name of the maps
                .map(|l| {
                    let numbers = parse_unsigned(l)
                        .iter()
                        .map(|&n| n as u64)
                        .collect::<Vec<u64>>();
                    [numbers[0], numbers[1], numbers[1] + numbers[2]]
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut seeds, maps) = parse_input(input);

    for map in &maps {
        for seed in &mut seeds {
            for &[to, start, end] in map {
                if start <= *seed && *seed < end {
                    *seed = *seed - start + to;
                    break;
                }
            }
        }
    }
    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_input(input);
    let mut seed_ranges: Vec<(u64, u64)> = seeds
        .chunks(2)
        .filter_map(|chunk| {
            if let [left, length] = chunk {
                Some((*left, *left + *length))
            } else {
                None
            }
        })
        .collect::<Vec<(u64, u64)>>();

    for map in &maps {
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        while let Some((seed_start, seed_end)) = seed_ranges.pop() {
            // Vector to check if we've found any matches
            let mut matches: Vec<(u64, u64)> = Vec::new();
            for &[to, start, end] in map {
                let overlap_start = seed_start.max(start);
                let overlap_end = seed_end.min(end);
                // We could only find matches if the overlap start is smaller than overlap end
                if overlap_start < overlap_end {
                    matches.push((overlap_start - start + to, overlap_end - start + to));
                    // Left hand side section
                    if overlap_start > seed_start {
                        seed_ranges.push((seed_start, overlap_start));
                    }
                    // Right hand side section
                    if seed_end > overlap_end {
                        seed_ranges.push((overlap_end, seed_end));
                    }
                    break;
                }
            }
            // If we haven't found any matches then map the range to itself
            if matches.is_empty() {
                new_ranges.push((seed_start, seed_end));
            } else {
                new_ranges.extend(matches);
            }
        }
        seed_ranges = new_ranges;
    }
    Some(
        seed_ranges
            .iter()
            .flat_map(|&(a, b)| vec![a, b])
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
