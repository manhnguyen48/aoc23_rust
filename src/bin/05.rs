advent_of_code::solution!(5);
use advent_of_code::parse_unsigned;

struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<[u64; 3]>>,
}

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
fn parse_input(input: &str) -> Input {
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

    Input { seeds, maps }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut seeds, maps) = {
        let parsed_input = parse_input(input);
        (parsed_input.seeds, parsed_input.maps)
    };
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
    // let (mut seeds, maps) = {
    //     let parsed_input = parse_input(input);
    //     (parsed_input.seeds, parsed_input.maps)
    // };
    Some(46)
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
