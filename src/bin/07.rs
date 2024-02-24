advent_of_code::solution!(7);

fn classify_hand(card_ranks: &[usize]) -> usize {
    // We're using an array length 15 to store frequencies of card ranks.
    let mut freq = [0; 15];
    card_ranks.iter().for_each(|&c| {
        freq[c] += 1;
    });
    // Turning off the count of jokers in the case its value is 1; in other cases this
    // value will be 0 any way.
    let jokers = freq[1];
    freq[1] = 0;
    freq.sort_unstable_by(|&a, &b| b.cmp(&a));
    // Jokers will always become whichever card is highest in frequency
    freq[0] += jokers;

    // Encoding the hand type as an integer by looking at the top 2 rank frequencies
    // and bit shifting them into the hand type.
    // Only the top 2 make the difference in hand type
    let mut hand_type = 0;
    for f in freq.iter().take(2) {
        hand_type = (hand_type << 4) | f;
    }
    hand_type
}

fn parse_hand(hand: &str, j_value: usize) -> usize {
    // Convert cards to its rank first
    let card_ranks: Vec<usize> = hand
        .bytes()
        .map(|b| match b {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => j_value,
            b'T' => 10,
            _ => (b - b'0') as usize,
        })
        .collect();
    // Get hand rank first
    let mut hand_value = classify_hand(&card_ranks);
    // Continue to bit shift to encode the rank info of the cards into the hand value
    for r in card_ranks {
        hand_value = (hand_value << 4) | r;
    }
    hand_value
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut data: Vec<(usize, u32)> = input
        .lines()
        .map(|line| {
            // First part is the hand, second part is the bid
            let mut parts = line.split_ascii_whitespace();
            (
                parse_hand(parts.next().unwrap(), 11),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    // sort by hand value to use the rank later
    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Some(
        data.iter()
            .enumerate()
            .map(|(i, &(_, bid))| ((i as u32) + 1) * bid)
            .sum(),
    )
}
// Everything the same as part 1, we just change the joker value to 1
pub fn part_two(input: &str) -> Option<u32> {
    let mut data: Vec<(usize, u32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (
                parse_hand(parts.next().unwrap(), 1),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();
    // sort by hand value to use the rank later
    data.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Some(
        data.iter()
            .enumerate()
            .map(|(i, &(_, bid))| ((i as u32) + 1) * bid)
            .sum(),
    )
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
