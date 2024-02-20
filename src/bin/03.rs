advent_of_code::solution!(3);
use hashbrown::HashSet;

/// Looks around the given position in the `lines` 2D array to find all
/// adjacent digits, storing their coordinates in a `Vec`. It will scan
/// left and right to extend digit sequences. Returns `None` if no digits
/// are found.
fn look_around(lines: &[&[u8]], i: usize, j: usize) -> Option<Vec<(usize, usize)>> {
    let mut coords: Vec<(usize, usize)> = Vec::new();

    for di in i.saturating_sub(1)..=(i + 1).min(lines.len() - 1) {
        for dj in j.saturating_sub(1)..=(j + 1).min(lines[0].len() - 1) {
            let mut start_j = dj;
            let line = &lines[di];
            // If the surrounding tile has a digit then we start traversing to find the begining of the digit sequence.
            if line[start_j].is_ascii_digit() {
                while &start_j > &0
                    && line[&start_j - 1].is_ascii_digit()
                    && line[dj].is_ascii_digit()
                {
                    start_j -= 1;
                }
                coords.push((di, start_j));
            }
        }
    }
    if coords.is_empty() {
        None
    } else {
        Some(coords)
    }
}

/// Converts the numbers at the given coordinates in the lines to a vector of u32 values.
/// Iterates through the coordinates, gets the line for each, and parses the number at that point,
/// converting it to a u32 and collecting into a vector.
fn convert_number(coords: &HashSet<(usize, usize)>, lines: &[&[u8]]) -> Vec<u32> {
    let numbers = coords
        .iter()
        .map(|(i, j)| {
            let line = &lines[*i];
            let number: u32 = line[*j..]
                .iter()
                .take_while(|&b| b.is_ascii_digit())
                .fold(0, |num, &b| num * 10 + ((b - b'0') as u32));
            number
        })
        .collect::<Vec<u32>>();
    numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut number_coords: HashSet<(usize, usize)> = HashSet::new();
    // Iterates through each line and character in the input, checking if the
    // character is not a dot or digit. If so, it calls look_around() to get
    // adjacent coordinates and adds them to the numbers HashSet. This populates
    // the numbers set with all coordinates of interest based on the criteria.
    lines.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, &b)| {
            if b != b'.' && !b.is_ascii_digit() {
                if let Some(n) = look_around(&lines, i, j) {
                    number_coords.extend(n);
                }
            }
        })
    });
    Some(convert_number(&number_coords, &lines).iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut result: u32 = 0;
    let mut gear_pairs: HashSet<(usize, usize)> = HashSet::new();
    // We only need to stop for the star symbol now
    lines.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, &b)| {
            if b == b'*' {
                if let Some(n) = look_around(&lines, i, j) {
                    // Using hashset to remove duplicates
                    gear_pairs.extend(n);
                    if gear_pairs.len() == 2 {
                        let gear_numbers = convert_number(&gear_pairs, &lines);
                        result += gear_numbers[0] * gear_numbers[1];
                    }
                    gear_pairs.clear();
                }
            }
        })
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4484));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467_835));
    }
}
