use ndarray::prelude::*;
use simple_grid::Grid;

/// Read a rectangular matrix of data from a string, where each byte encodes a single value
/// and rows are separated by newlines. Extra whitespace is allowed both around the grid and between the rows.
/// # Panics
/// * if `s` doesn't contain at least one row
/// * if rows have different lengths
pub fn read_grid(s: &str) -> Grid<u8> {
    read_grid_with(s, |b| *b)
}

/// Read a rectangular matrix of data from a string, where each byte encodes a single value
/// with a user supplied translation function, and rows are separated by newlines.
/// Extra whitespace is allowed both around the grid and between the rows.
/// # Panics
/// * if `s` doesn't contain at least one row
/// * if rows have different lengths
pub fn read_grid_with<T>(s: &str, transform: impl Fn(&u8) -> T) -> Grid<T> {
    let lines: Vec<_> = s.trim().lines().collect();
    assert!(!lines.is_empty(), "grid can't be empty");
    let h = lines.len();
    let w = lines[0].len();
    let data = lines
        .into_iter()
        .flat_map(|s| s.trim().as_bytes())
        .map(transform)
        .collect();
    Grid::new(w, h, data)
}

/// Read a rectangular matrix of data from a string, where each byte encodes a single value
/// and rows are separated by newlines. Extra whitespace is allowed both around the grid and between the rows.
/// # Panics
/// * if `s` doesn't contain at least one row
/// * if rows have different lengths
pub fn read_2d_array(s: &str) -> Array2<u8> {
    read_2d_array_with(s, |b| *b)
}

/// Read a rectangular matrix of data from a string, where each byte encodes a single value
/// with a user supplied translation function, and rows are separated by newlines.
/// Extra whitespace is allowed both around the ndarray and between the rows.
/// # Panics
/// * if `s` doesn't contain at least one row
/// * if rows have different lengths
pub fn read_2d_array_with<T>(s: &str, transform: impl Fn(&u8) -> T) -> Array2<T> {
    let lines: Vec<_> = s.trim().lines().collect();
    let h = lines.len();
    let w = lines[0].len();
    let data = lines
        .into_iter()
        .flat_map(|s| s.trim().as_bytes())
        .map(transform)
        .collect();
    Array2::from_shape_vec((h, w), data).unwrap()
}

/// Lowest Common Multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Greatest Common Divisor
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
