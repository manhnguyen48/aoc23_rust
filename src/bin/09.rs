advent_of_code::solution!(9);

/// Recursively predicts the next number in a sequence by calculating
/// differences between adjacent numbers. `numbers` is the input sequence,
/// `init` is the starting prediction. At each step the last difference
/// is added to the previous prediction to generate the next prediction.
/// Recurses until differences are all 0.
fn predict(numbers: &[i32], init: i32) -> i32 {
    let diffs: Vec<i32> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let init = init + diffs[diffs.len() - 1];

    if diffs.iter().any(|a| *a != 0) {
        predict(&diffs, init)
    } else {
        init
    }
}

fn find_value(input: &str, backward: bool) -> Option<i32> {
    let results: i32 = input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect::<Vec<i32>>();
            if backward {
                numbers.reverse()
            }
            predict(&numbers, *numbers.last().unwrap())
        })
        .sum();
    Some(results)
}

pub fn part_one(input: &str) -> Option<i32> {
    find_value(input, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    find_value(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
