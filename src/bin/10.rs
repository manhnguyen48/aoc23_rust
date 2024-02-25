advent_of_code::solution!(10);
use advent_of_code::utils::matrix::*;

/// Calculates the area inside a polygon defined by a vector of coordinate pairs using the shoelace formula.
///
/// Appends the first coordinate to the end to close the loop. Calculates the area inside using the shoelace
/// formula on the coordinate pairs. Adjusts the raw area to account for the polygon sides.
fn shoelace(coordinates: Vec<(i32, i32)>) -> u32 {
    // Calculate the area using the shoelace formula
    let area = 0.5
        * (coordinates
            .iter()
            .zip(coordinates.iter().skip(1))
            .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
            .sum::<i32>()
            .abs() as f64);

    // Calculate the inside value
    let inside = area - ((coordinates.len() - 1) as f64 / 2.0) + 1.0;

    inside as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Matrix::parse(input);
    let tile = grid.find_one(b'S')?;

    // Finding the appropriate first step, we either can go up or down
    let mut direction = match grid.in_bounds(&(tile + N)) {
        true if matches!(grid[tile + N], b'|' | b'7' | b'F') => N,
        true => S,
        false if matches!(grid[tile + S], b'|' | b'J' | b'L') => S,
        _ => N,
    };
    // A single mutable coordinate pair to traverse the grid
    let mut pos = tile + direction;
    let mut steps: u32 = 1;

    loop {
        // If we run into a straight pipe then keep going in that direction
        while grid[pos] == b'-' || grid[pos] == b'|' {
            pos += direction;
            steps += 1;
        }
        // Otherwise change direction here
        direction = match grid[pos] {
            // Changing direction from vertical to horizontal
            b'7' if direction == N => W,
            b'F' if direction == N => E,
            b'J' if direction == S => W,
            b'L' if direction == S => E,
            // Changing direction from horizontal to vertical
            b'J' | b'L' => N,
            b'7' | b'F' => S,
            _ => break,
        };
        pos += direction;
        steps += 1;
    }
    Some(steps / 2)
}

// Difference for part 2 is we don't care about steps, but collecting the coordinates
// of polygon perimeter then calculate the area inside
pub fn part_two(input: &str) -> Option<u32> {
    let grid = Matrix::parse(input);
    let tile = grid.find_one(b'S')?;
    let mut traversal_order: Vec<(i32, i32)> = Vec::new();
    traversal_order.push((tile.x, tile.y));

    // Finding the appropriate first step, we either can go up or down
    let mut direction = match grid.in_bounds(&(tile + N)) {
        true if matches!(grid[tile + N], b'|' | b'7' | b'F') => N,
        true => S,
        false if matches!(grid[tile + S], b'|' | b'J' | b'L') => S,
        _ => N,
    };

    // A single mutable coordinate pair to traverse the grid
    let mut pos = tile + direction;

    loop {
        // If we run into a straight pipe then keep going in that direction
        while grid[pos] == b'-' || grid[pos] == b'|' {
            traversal_order.push((pos.x, pos.y));
            pos += direction;
        }
        // Otherwise change direction here
        direction = match grid[pos] {
            b'7' if direction == N => W,
            b'F' if direction == N => E,
            b'J' if direction == S => W,
            b'L' if direction == S => E,
            b'J' | b'L' => N,
            b'7' | b'F' => S,
            _ => break,
        };
        traversal_order.push((pos.x, pos.y));
        pos += direction;
    }
    // Append the first coordinate to complete the loop
    traversal_order.push(traversal_order[0]);
    Some(shoelace(traversal_order))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
