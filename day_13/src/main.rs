use ndarray as nd;

fn main() {
    problem_one();
}

fn problem_one() {
    let problem_text = include_str!("problem_text");
    let axis_totals = get_axis_totals(problem_text);

    println!("Problem 1: {axis_totals}")
}

fn get_axis_totals(input: &str) -> usize {
    let arrays = parse_array_list(input);

    arrays
        .iter()
        .filter_map(scan_array)
        .fold(0, |acc, x| acc + x.value())
}

#[derive(Debug, PartialEq)]
enum SymmetryAxis {
    Row(usize),
    Col(usize),
}

impl SymmetryAxis {
    fn value(&self) -> usize {
        match &self {
            SymmetryAxis::Row(x) => 100 * x,
            SymmetryAxis::Col(x) => *x,
        }
    }
}

struct IndexPair {
    left: usize,
    right: usize,
    max_val: usize,
}

impl IndexPair {
    fn from_index(index: usize, max_val: usize) -> Option<Self> {
        let left = index;
        let right = index + 1;
        if right >= max_val {
            return None;
        }

        Some(IndexPair {
            left,
            right,
            max_val,
        })
    }

    /// Increase the spacing of the columns, returning None if we go beyond the edge of the array
    fn expand(&self) -> Option<Self> {
        if self.left == 0 || self.right >= self.max_val - 1 {
            return None;
        };
        Some(IndexPair {
            left: self.left - 1,
            right: self.right + 1,
            max_val: self.max_val,
        })
    }
}

/// Search for a symmetry axis in the array block
///
/// We iterate through the col (and row) of an array, if the selected column matches the next column
/// then it is a candidate to be a symettry axis. We then "expand" the column pairs and keep checking
/// if the columns are identical. If a pair is not identical, then we give up on this candinate.
/// Otherwise, if we hit the edge of the array then this is an axis of symmetry.
fn scan_array(arr: &nd::Array2<bool>) -> Option<SymmetryAxis> {
    let row_axis = scan_rows(arr);
    if let Some(row) = row_axis {
        return Some(SymmetryAxis::Row(row));
    }

    let col_axis = scan_cols(arr);
    if let Some(col) = col_axis {
        return Some(SymmetryAxis::Col(col));
    }

    return None;
}

fn scan_cols(arr: &nd::Array2<bool>) -> Option<usize> {
    let n_cols = arr.ncols();

    for col_index in 0..n_cols - 1 {
        let mut col_pair = IndexPair::from_index(col_index, n_cols).expect("Invalid main col pair");
        let col_left = arr.column(col_pair.left);
        let col_right = arr.column(col_pair.right);

        // If we don't match the next col this isn't a mirror point so continue
        if col_left != col_right {
            continue;
        }

        loop {
            if let Some(cols) = col_pair.expand() {
                col_pair = cols;
                let col_left = arr.column(col_pair.left);
                let col_right = arr.column(col_pair.right);

                // Candidate is not a true symmetry axis
                if col_right != col_left {
                    break;
                }
            } else {
                // Here we hit the edges of the array while matching all previous mirror pairs
                // So this is the symmetry axis
                return Some(col_index + 1);
            }
        }
    }

    None
}

fn scan_rows(arr: &nd::Array2<bool>) -> Option<usize> {
    // TODO: Merge this with the scan cols function
    let n_rows = arr.nrows();

    for row_index in 0..n_rows - 1 {
        let mut row_pair = IndexPair::from_index(row_index, n_rows).expect("Invalid main row pair");
        let row_left = arr.row(row_pair.left);
        let row_right = arr.row(row_pair.right);

        // If we don't match the next row this isn't a mirror point so continue
        if row_left != row_right {
            continue;
        }

        loop {
            if let Some(rows) = row_pair.expand() {
                row_pair = rows;
                let row_left = arr.row(row_pair.left);
                let row_right = arr.row(row_pair.right);

                if row_right != row_left {
                    break;
                }
            } else {
                // Here we hit the edges of the array while matching all previous mirror pairs
                return Some(row_index + 1);
            }
        }
    }

    None
}

/// Parse a single contigous pattern block into an array
fn parse_array(input: &str) -> nd::Array2<bool> {
    let rows: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let row_count = rows.len();
    let col_count = rows[0].len();

    let mut flat_vec = Vec::with_capacity(row_count * col_count);

    for row in rows {
        flat_vec.extend(row);
    }

    nd::Array2::from_shape_vec((row_count, col_count), flat_vec)
        .expect("Unable to convert vecs into array")
}

fn parse_array_list(input: &str) -> Vec<nd::Array2<bool>> {
    input.split("\n\n").map(parse_array).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample_one() {
        let test_data = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "};

        let parsed_array = parse_array(test_data);

        let parsed_array_actual = nd::arr2(&[
            [true, false, true, true, false, false, true, true, false],
            [false, false, true, false, true, true, false, true, false],
            [true, true, false, false, false, false, false, false, true],
            [true, true, false, false, false, false, false, false, true],
            [false, false, true, false, true, true, false, true, false],
            [false, false, true, true, false, false, true, true, false],
            [true, false, true, false, true, true, false, true, false],
        ]);

        assert_eq!(parsed_array, parsed_array_actual)
    }

    #[test]
    fn test_scan_col() {
        let test_data = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "};

        let parsed_array = parse_array(test_data);
        let symmetry_col = scan_cols(&parsed_array);

        assert_eq!(symmetry_col, Some(5));
    }

    #[test]
    fn test_scan_row() {
        let test_data = indoc! {"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};
        let parsed_array = parse_array(test_data);
        let symmetry_row = scan_rows(&parsed_array);

        assert_eq!(symmetry_row, Some(4));

        let symmetry_col = scan_cols(&parsed_array);
        assert_eq!(symmetry_col, None);
    }

    #[test]
    fn test_scan() {
        let test_data = indoc! {"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};
        let parsed_array = parse_array(test_data);
        let symmetry_axis = scan_array(&parsed_array);

        assert_eq!(symmetry_axis, Some(SymmetryAxis::Row(4)));
    }

    #[test]
    fn test_sample_one() {
        let test_data = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};
        let sample_one = get_axis_totals(test_data);

        assert_eq!(sample_one, 405)
    }
}
