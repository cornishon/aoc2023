use simple_grid::Grid;

/// Read a rectangular matrix of data from a string, where each byte encodes a single value
/// and rows are separated by newlines. Extra whitespace is allowed both around the grid and between the rows.
/// # Panics
/// * if `s` doesn't contain at least one row
/// * if rows have different lengths
pub fn read_grid<T>(s: &str, from_u8: impl Fn(&u8) -> T) -> Grid<T> {
    let lines: Vec<_> = s.trim().lines().collect();
    let h = lines.len();
    let w = lines[0].len();
    let data = lines
        .into_iter()
        .flat_map(|s| s.trim().as_bytes())
        .map(from_u8)
        .collect();
    Grid::new(w, h, data)
}
