advent_of_code::solution!(2);

struct Game {
    r: u32,
    g: u32,
    b: u32,
}

impl Game {
    fn product(&self) -> u32 {
        self.r * self.g * self.b
    }
    fn min_viable(&self, r: u32, g: u32, b: u32) -> bool {
        self.r <= r && self.g <= g && self.b <= b
    }
}

fn parse_input(line: &str) -> Option<Game> {
    let mut game = Game { r: 0, g: 0, b: 0 };
    line.split_once(": ")?
        .1
        .split([',', ';'])
        .try_for_each(|color| {
            let (count_s, color_s) = color.trim().split_once(' ')?;
            let count = count_s.parse().ok()?;
            match color_s.as_bytes().first()? {
                b'r' => {
                    game.r = std::cmp::max(count, game.r);
                }
                b'g' => {
                    game.g = std::cmp::max(count, game.g);
                }
                b'b' => {
                    game.b = std::cmp::max(count, game.b);
                }
                _ => unreachable!(),
            }
            Some(())
        });

    Some(game)
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            parse_input(line).and_then(|game| {
                if game.min_viable(12, 13, 14) {
                    Some(i + 1)
                } else {
                    None
                }
            })
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .filter_map(|line| parse_input(line).map(|game| game.product()))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
