pub mod template;

// Use this file to add helper functions and additional modules.

/// Calculates the greatest common divisor (GCD) of two unsigned integers a and b
/// using Euclid's algorithm.
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

/// Calculates the least common product of the elements in the given slice `nums`.
/// Uses recursion and Euclid's algorithm for GCD.
pub fn lcp(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcp(&nums[1..]);
    (a * b) / gcd(a, b)
}
