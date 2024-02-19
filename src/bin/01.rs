advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let result = input
        .lines()
        .map(|line| {
            let mut digits = line
                .bytes()
                // filter out non-digits and convert back to u8
                .filter(|b| b.is_ascii_digit())
                .map(|b| b - b'0');

            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);
            (first * 10 + last) as i32
        })
        .sum();
    Some(result)
}

const MAPPING: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];
pub fn part_two(input: &str) -> Option<i32> {
    // Replace the spelt out digits with number
    let replaced = &MAPPING
        .iter()
        .fold(input.to_string(), |acc, (old, new)| acc.replace(old, new));
    // After this we have the same problem as part 1
    part_one(replaced)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
