advent_of_code::solution!(13);
use advent_of_code::utils::matrix::*;

/// Parses the input string into a vector of grids, where each grid is a vector of rows and columns.
/// Each row and column is a vector of bits representing the presence or absence of a '#' in the grid.
fn parse_input(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let grids = input
        .split("\n\n")
        .map(|block| {
            let grid = Matrix::parse(block);
            let mut rows = Vec::new();
            let mut cols = Vec::new();

            for i in 0..grid.height {
                let mut n = 0;
                for j in 0..grid.width {
                    n = (n << 1) | (grid[Point::new(j, i)] == b'#') as usize;
                }
                rows.push(n);
            }

            for j in 0..grid.width {
                let mut m = 0;
                for i in 0..grid.height {
                    m = (m << 1) | (grid[Point::new(j, i)] == b'#') as usize;
                }
                cols.push(m);
            }
            (rows, cols)
        })
        .collect::<Vec<(Vec<usize>, Vec<usize>)>>();
    grids
}
/// Function to find the reflection of a grid in a given direction.
/// we also keep track of the smudge count (to see if the relfection can be found if it's just off by one)
/// This means part 2 is just a special case of part 1.
fn find_symmetry(numbers: &[usize], smudge_count: usize) -> Option<usize> {
    let length = numbers.len();
    // Start comparing pairs of numbers from the second numbers onwards
    for i in 1..length {
        let mut reflection = 0;
        for j in 0..i.min(length - i) {
            // Count the number of bits that are different between the two numbers using XOR
            // If 2 numbers are the same, the XOR will be 0 else we can keep looping
            reflection += (numbers[i - j - 1] ^ numbers[i + j]).count_ones() as usize;
        }
        if reflection == smudge_count {
            return Some(i);
        }
    }
    // We might not find any reflections
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse_input(input);

    let results: usize = grids
        .iter()
        .map(|(rows, cols)| {
            if let Some(row) = find_symmetry(rows, 0) {
                row * 100
            } else if let Some(col) = find_symmetry(cols, 0) {
                col
            } else {
                // There must be always a reflection be it row or column
                unreachable!()
            }
        })
        .sum();
    Some(results)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grids = parse_input(input);

    let results: usize = grids
        .iter()
        .map(|(rows, cols)| {
            if let Some(row) = find_symmetry(rows, 1) {
                row * 100
            } else if let Some(col) = find_symmetry(cols, 1) {
                col
            } else {
                unreachable!()
            }
        })
        .sum();
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
