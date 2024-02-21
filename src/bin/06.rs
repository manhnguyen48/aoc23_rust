advent_of_code::solution!(6);
use advent_of_code::parse_unsigned;

// Solving the quadratic equation for the number of ways to win the race
fn solve(times: &[f64], max_scores: &[f64]) -> f64 {
    times
        .iter()
        .zip(max_scores)
        .map(|(t, sc)| {
            let x = t * t - 4.0 * sc;
            let a = (t + x.sqrt()) / 2.0;
            let b = (t - x.sqrt()) / 2.0;
            a.ceil() - b.floor() - 1.0
        })
        .product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed_input = input
        .lines()
        .map(|line|
            parse_unsigned(line)
                .iter()
                .map(|&n| n as f64)
                .collect()
        )
        .collect::<Vec<Vec<f64>>>();

    Some(solve(&parsed_input[0], &parsed_input[1]) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed_input = input
        .lines()
        .filter_map(|l| {
            l.split_once(':').and_then(|l| {
                String::from_iter(l.1.chars().filter(|c| !c.is_whitespace()))
                    .parse::<f64>()
                    .ok()
            })
        })
        .collect::<Vec<f64>>();

    Some(solve(&[parsed_input[0]], &[parsed_input[1]]) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71_503));
    }
}
