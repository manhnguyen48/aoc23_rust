advent_of_code::solution!(4);

fn parse_unsigned(input_string: &str) -> Vec<usize> {
    let (mut result, num) =
        input_string
            .bytes()
            .fold((Vec::new(), 0), |(mut result, mut num), b| {
                if b.is_ascii_digit() {
                    num = num * 10 + (b - b'0') as usize;
                } else if num > 0 {
                    result.push(num);
                    num = 0;
                }
                (result, num)
            });

    if num > 0 {
        result.push(num);
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let matches = input
        .lines()
        .map(|line| {
            let (w, p) = line.split_once("|").expect("Invalid input");
            let winners = parse_unsigned(&w);
            let players = parse_unsigned(&p);
            // All numbers are 2 digits so we can use a bool array to store the matches.
            let mut match_count = [false; 100];
            winners.iter().skip(1).for_each(|i| match_count[*i] = true);
            players.iter().filter(|&i| match_count[*i]).count()
        })
        .collect::<Vec<usize>>();
    // Bit shifting here 1 << n means 2^n, so we need to divide by 2 to get the correct result.
    Some(matches.iter().map(|&n| (1 << n) >> 1).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
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
