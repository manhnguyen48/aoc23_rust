advent_of_code::solution!(11);

// We don't actually need to calculate the manhattan distance
// but we're counting the number of galaxies in each row and column
// then expand through the count by a factor of n and sum the results

fn distance(counts: &[usize], factor: usize) -> usize {
    let (mut gaps, mut total, mut items, mut dist) = (0, 0, 0, 0);
    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            let expansion = i + factor * gaps;
            let extra = items * expansion - total; // extra galaxies in the expansion to avoid overflow
            dist += count * extra;
            total += expansion * count;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}

pub fn part_one(input: &str) -> Option<u32> {
    let size = input.bytes().position(|b| b == b'\n').unwrap();

    // Count the number of galaxies in each row and column
    let mut verticals = vec![0; size];
    let mut horizontals = vec![0; size];

    input
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b == b'#')
        .for_each(|(i, _)| {
            verticals[i % (size + 1)] += 1;
            horizontals[i / (size + 1)] += 1;
        });

    Some((distance(&verticals, 1) + distance(&horizontals, 1)) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let size = input.bytes().position(|b| b == b'\n').unwrap();

    // Count the number of galaxies in each row and column
    let mut verticals = vec![0; size];
    let mut horizontals = vec![0; size];

    input
        .bytes()
        .enumerate()
        .filter(|(_, b)| *b == b'#')
        .for_each(|(i, _)| {
            verticals[i % (size + 1)] += 1;
            horizontals[i / (size + 1)] += 1;
        });

    Some((distance(&verticals, 999_999) + distance(&horizontals, 999_999)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
