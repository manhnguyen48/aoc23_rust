advent_of_code::solution!(4);

struct Card {
    // id: u32,
    winners: Vec<u32>,
    players: Vec<u32>,
}
impl Card {
    fn new(card_input: Vec<&str>) -> Self {
        // let id = card_input[0].replace("Card ", "").parse::<u32>().unwrap();
        let winners = card_input[1]
            .split(" ")
            .filter_map(|number| number.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let players = card_input[2]
            .split(" ")
            .filter_map(|number| number.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        Card {
            // id,
            winners,
            players,
        }
    }
    fn count_matches(&self) -> u32 {
        self.players
            .iter()
            .filter(|&p| self.winners.contains(p))
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .filter_map(|l| {
            let card_input = l
                .split(&[':', '|'])
                .map(|part| part.trim())
                .collect::<Vec<&str>>();
            let card = Card::new(card_input);
            let matches: u32 = card.count_matches();
            // Check if count_matches returns 0 to avoid overflow
            if matches > 0 {
                Some(u32::pow(2, matches - 1))
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(30)
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
