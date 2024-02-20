advent_of_code::solution!(4);

// Parses a vector of integers from a string.
//
// Splits the string on non-digit bytes, converting digit chunks to numbers.
// Accumulates the numbers into a vector and return it. Somehow for loops are faster than iterators on AMD compared to Intel.
fn parse_unsigned(input_string: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let mut num = 0;
    for b in input_string.bytes() {
        if b.is_ascii_digit() {
            num = num * 10 + ((b - b'0') as usize);
        } else if num != 0 {
            result.push(num);
            num = 0;
        }
    }
    if num != 0 {
        result.push(num);
    }
    result
}

// Parses input containing winners and players on each line, counting how
// many players have winning numbers.
//
// Splits each line on the '|' delimiter, parsing the first section into
// winner numbers and the second into player numbers. Uses a bool array to
// track winner numbers. Filters player numbers based on winners, counting
// matches.
fn get_matches(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (winners, players) = {
                let (w, p) = line.split_once('|').unwrap();
                (parse_unsigned(w), parse_unsigned(p))
            };
            // All numbers are 2 digits so we can use a bool array to store the matches.
            let mut match_count = [false; 100];
            winners.iter().skip(1).for_each(|i| {
                match_count[*i] = true;
            });
            players.iter().filter(|&i| match_count[*i]).count()
        })
        .collect::<Vec<usize>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matches = get_matches(input);
    // Bit shifting here 1 << n means 2^n, so we need to divide by 2 to get the correct result.
    Some(matches.iter().map(|&n| (1 << n) >> 1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let matches = get_matches(input);
    // We get 1 copy of each card
    let mut cards = vec![1; matches.len()];
    // Iterating through the matches while keeping track of
    // index of which card to add
    for (i, &n) in matches.iter().enumerate() {
        (1..=n).for_each(|j| cards[i + j] += cards[i]);
    }
    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
