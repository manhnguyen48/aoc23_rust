// Parses a vector of integers from a string.
//
// Splits the string on non-digit bytes, converting digit chunks to numbers.
// Accumulates the numbers into a vector and return it. Somehow for loops are faster than iterators on AMD compared to Intel.
pub fn parse_unsigned(input_string: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let mut num = 0;
    let mut digit_found = false;
    for b in input_string.bytes() {
        match b.is_ascii_digit() {
            true => {
                num = num * 10 + ((b - b'0') as usize);
                digit_found = true;
            }
            false if digit_found => {
                result.push(num);
                num = 0;
                digit_found = false;
            }
            _ => {}
        }
    }
    if digit_found {
        result.push(num);
    }
    result
}
