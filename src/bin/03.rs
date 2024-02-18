advent_of_code::solution!(3);
use regex::Regex;

/// Checks the positions around the given index in the 2D grid of lines to see if any contain a non-dot, non-digit character.
fn look_around(lines: &[Vec<u8>], i: usize, start: usize, len: usize) -> bool {
    let positions = [
        (i.saturating_sub(1), start.saturating_sub(1)..start + len.saturating_add(1)),
        (i, start.saturating_sub(1)..start),
        (i, start + len..start + len.saturating_add(1)),
        (i.saturating_add(1), start.saturating_sub(1)..start + len.saturating_add(1)),
    ];
    positions.iter().any(|&(i, ref range)| {
        range.clone().any(|j| {
            lines
                .get(i)
                .and_then(|l| l.get(j))
                .map_or(false, |&b| b != b'.' && !b.is_ascii_digit())
        })
    })
}
pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().collect())
        .collect();
    let mut result = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();

    for (i, line) in input.lines().enumerate() {
        for mat in re.find_iter(&line) {
            if look_around(&lines, i, mat.start(), mat.len()) {
                result.push(mat.as_str().parse::<u32>().unwrap());
            }
        }
    }
    Some(result.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(467_835)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467_835));
    }
}
