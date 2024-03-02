use advent_of_code::utils::parsing::parse_unsigned;
use fxhash::FxHashMap;
use rayon::prelude::*;

advent_of_code::solution!(12);

struct Spring {
    springs: Vec<u8>,
    config: Vec<usize>,
}

impl Spring {
    fn new(springs: Vec<u8>, config: Vec<usize>) -> Self {
        Self { springs, config }
    }
}

/// This function counts the number of valid spring configurations based on a given `Spring` object.
/// It uses a memoization technique with a hash map `memo` to avoid redundant calculations.
///
/// # Arguments
///
/// * `spring`: A reference to the `Spring` object containing configuration data.
/// * `memo`: A mutable reference to a hash map storing previously calculated results.
/// * `pos`: The current position in the spring sequence.
/// * `block`: The current accumulated block size.
/// * `sequences`: The number of completed sequences so far.
///
/// # Returns
///
/// The number of valid spring configurations starting from the current position.
fn count(
    spring: &Spring,
    memo: &mut FxHashMap<(usize, usize, usize), usize>,
    pos: usize,
    block: usize,
    sequences: usize,
) -> usize {
    if let Some(&result) = memo.get(&(pos, block, sequences)) {
        return result;
    }

    let result = if pos == spring.springs.len() {
        (sequences == spring.config.len()) as usize
    } else if spring.springs[pos] == b'#' {
        count(spring, memo, pos + 1, block + 1, sequences)
    } else if spring.springs[pos] == b'.' || sequences == spring.config.len() {
        if sequences < spring.config.len() && block == spring.config[sequences] {
            count(spring, memo, pos + 1, 0, sequences + 1)
        } else if block == 0 {
            count(spring, memo, pos + 1, 0, sequences)
        } else {
            0
        }
    } else {
        let mut result = count(spring, memo, pos + 1, block + 1, sequences);
        if block == spring.config[sequences] {
            result += count(spring, memo, pos + 1, 0, sequences + 1)
        } else if block == 0 {
            result += count(spring, memo, pos + 1, 0, sequences)
        }
        result
    };

    memo.insert((pos, block, sequences), result);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let (springs, config) = line.split_once(' ').unwrap();
            let config = parse_unsigned(config);
            let springs = springs
                .bytes()
                .chain(std::iter::once(b'.'))
                .collect::<Vec<_>>();

            count(
                &Spring::new(springs, config),
                &mut FxHashMap::default(),
                0,
                0,
                0,
            )
        })
        .sum::<usize>();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .par_lines()
        .map(|line| {
            let (springs, config) = line.split_once(' ').unwrap();
            let config = parse_unsigned(config).repeat(5);
            let springs = springs
                .bytes()
                .chain(std::iter::once(b'?'))
                .collect::<Vec<_>>()
                .repeat(5);

            count(
                &Spring::new(springs, config),
                &mut FxHashMap::default(),
                0,
                0,
                0,
            )
        })
        .sum::<usize>();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
