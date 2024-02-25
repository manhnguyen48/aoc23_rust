use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, IndexMut};

// Point struct to represent a point in a matrix.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

// Movements
pub const N: Point = Point { x: 0, y: -1 };
pub const S: Point = Point { x: 0, y: 1 };
pub const W: Point = Point { x: -1, y: 0 };
pub const E: Point = Point { x: 1, y: 0 };
pub const NE: Point = Point { x: 1, y: -1 };
pub const NW: Point = Point { x: -1, y: -1 };
pub const SE: Point = Point { x: 1, y: 1 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const CARDINAL: [Point; 4] = [N, S, E, W];

// Addition implementation to allow us to move points around.
impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    pub width: i32,
    pub height: i32,
    pub contents: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

// Indexing implementations to allow us getting the contents of the matrix by indexing it with a Point.
// Will need bound checking before use.
impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.contents[(point.y * self.width + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.contents[(point.y * self.width + point.x) as usize]
    }
}

// For parsing a string into a matrix of bytes.
impl Matrix<u8> {
    /// Parses the given string input into a Matrix of u8 values.
    /// Splits the string into lines, converts each line into bytes,
    /// and populates the Matrix width, height and contents fields accordingly.
    pub fn parse(input: &str) -> Self {
        let raw_bytes: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
        let width = raw_bytes[0].len() as i32;
        let height = raw_bytes.len() as i32;
        let mut contents = Vec::new();
        raw_bytes.iter().for_each(|b| contents.extend_from_slice(b));
        Matrix {
            width,
            height,
            contents,
        }
    }

    /// Searches the matrix for the given `item` and returns the `Point` location if found.
    /// Returns `None` if the item is not found in the matrix.
    pub fn find_one(&self, item: u8) -> Option<Point> {
        self.contents
            .iter()
            .position(|&b| b == item)
            .map(|idx| Point::new((idx as i32) % self.width, (idx as i32) / self.width))
    }
    /// Searches the matrix for the given `item` and returns the all the locations if found.
    /// Returns an empty vector if the item is not found in the matrix.
    pub fn find_all(&self, item: u8) -> Vec<Point> {
        self.contents
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| {
                if b == item {
                    Some(Point::new(
                        (idx as i32) % self.width,
                        (idx as i32) / self.width,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}
