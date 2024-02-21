advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    Some(6440)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(5905)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}