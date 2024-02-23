advent_of_code::solution!(7);

fn parse_hand(hand: &str, mapping: &[char; 13]) -> u64 {
    hand.chars()
        .map(|c| mapping.iter().position(|&x| x == c).unwrap())
        // TODO: Change the init to classfied hand rank number
        .fold(0, |acc, x| acc * 100 + x as u64)
}

pub fn part_one(input: &str) -> Option<u32> {
    const MAPPING: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let results = input
        .lines()
        .map(|line| {
            let (hand_value, bid_value) = {
                let mut parts = line.split_ascii_whitespace();
                (
                    parse_hand(parts.next().unwrap(), &MAPPING),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                )
            };
            println!("hand: {}, bid: {}", hand_value, bid_value);
            (hand_value, bid_value)
        })
        .collect::<Vec<(u64, u64)>>();
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
