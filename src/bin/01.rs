advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
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
            (first * 10 + last) as u32
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
