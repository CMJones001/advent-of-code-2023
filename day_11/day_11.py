#!/usr/bin/env python
import unittest
from typing import List

import numpy as np
from scipy.spatial.distance import cdist, pdist, squareform


def problem_one():
    with open("problem_text") as f:
        input_ = f.read()

    min_distance = get_min_distance(input_)
    print(f"Problem 1 output: {min_distance}")

    expected = 10231178
    if min_distance != expected:
        print(f"Test 1 failed. Expected {expected}, got {min_distance}")


def problem_two():
    with open("problem_text") as f:
        input_ = f.read()

    expansion = 1_000_000
    min_distance = get_min_distance_large(input_, expansion)
    print(f"Problem 2 output: {min_distance}")


def get_min_distance_large(input_: str, expansion=1) -> int:
    """Get the sum of all minimum distance pairs in the galaxy array.

    This makes use of the fact that the distance sum is linearly increasing with the expansion factor.
    Enabling us to calculate the distance sum for large expansion factors without having to expand the array.
    """
    expansion_one = get_min_distance(input_, 1)
    expansion_two = get_min_distance(input_, 2)

    delta = expansion_two - expansion_one
    expansion_n = delta * (expansion - 2) + expansion_one
    return expansion_n


def get_min_distance(input: str, expansion=1) -> int:
    """Get the sum of all minimum distance pairs in the galaxy array.

    Don't use this with large expansion values, it's very slow.
    """
    galaxy_array = parse_input(input)
    expanded_array = expand_array_gen(galaxy_array, expansion)
    galaxies = locate_galaxies(expanded_array)
    distances = get_galaxy_distances(galaxies)

    return distances.sum()


def parse_input(input: str) -> np.ndarray:
    """Parse the input into a numpy array."""

    def format_line(line: str) -> list[bool]:
        return [l == "#" for l in line.strip()]

    return np.array([format_line(line) for line in input.strip().split("\n")])


def expand_array(arr: np.ndarray) -> np.ndarray:
    """For each empty row and column, insert a row/column there."""
    empty_rows = arr.sum(axis=1) == 0
    empty_cols = arr.sum(axis=0) == 0

    new_arr = np.insert(arr, empty_rows.nonzero()[0], 0, axis=0)
    new_arr = np.insert(new_arr, empty_cols.nonzero()[0], 0, axis=1)

    return new_arr


def expand_array_gen(arr: np.ndarray, expansion: int) -> np.ndarray:
    """For each empty row and column, insert ``expansion`` row/columns there."""
    empty_rows = (arr.sum(axis=1) == 0).ravel()
    empty_cols = (arr.sum(axis=0) == 0).ravel()

    n_rows, n_cols = arr.shape
    n_rows_new = n_rows + empty_rows.sum() * expansion
    n_cols_new = n_cols + empty_cols.sum() * expansion

    row_expanded = np.zeros((n_rows_new, n_cols))

    row_index = 0
    for old_row, is_empty in enumerate(empty_rows):
        if is_empty:
            row_index += expansion + 1
        else:
            row_expanded[row_index] = arr[old_row]
            row_index += 1

    col_expanded = np.zeros((n_rows_new, n_cols_new))

    col_index = 0
    for old_col, is_empty in enumerate(empty_cols):
        if is_empty:
            col_index += expansion
        else:
            col_expanded[:, col_index] = row_expanded[:, old_col]
        col_index += 1

    return col_expanded


def locate_galaxies(arr: np.ndarray) -> np.ndarray:
    """Get the locations of all galaxies in the array."""
    galaxies = np.argwhere(arr)
    return galaxies


def get_galaxy_distances(galaxies: np.ndarray) -> np.ndarray:
    """Get the distances between all galaxies."""
    distances = squareform(cdist(galaxies, galaxies, metric="cityblock"))
    distances = distances[distances != 0]
    return distances.astype(int)


if __name__ == "__main__":
    problem_one()
    problem_two()


class Tests(unittest.TestCase):
    def setUp(self):
        self.test_input = """
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            """

    def test_expand_array(self):
        base_array = parse_input(self.test_input)

        expanded_array = expand_array(base_array)

        self.assertEqual(expanded_array.shape, (12, 13))

    def test_locate_galaxies(self):
        base_array = parse_input(self.test_input)
        expanded_array = expand_array(base_array)

        galaxies = locate_galaxies(expanded_array)
        self.assertEqual(galaxies.shape, (9, 2))

    def test_get_galaxy_distances(self):
        base_array = parse_input(self.test_input)
        expanded_array = expand_array(base_array)

        galaxies = locate_galaxies(expanded_array)
        distances = get_galaxy_distances(galaxies)

        self.assertEqual(distances.shape, (36,))

    def test_problem_one(self):
        base_array = parse_input(self.test_input)
        expanded_array = expand_array(base_array)

        galaxies = locate_galaxies(expanded_array)
        distances = get_galaxy_distances(galaxies)
        distance_sum = distances.sum()
        self.assertEqual(distance_sum, 374)

    def test_expansion_n(self) -> np.ndarray:
        self.assertEqual(get_min_distance_large(self.test_input, 10), 1030)
        self.assertEqual(get_min_distance_large(self.test_input, 100), 8410)


class TestGenExpansion(unittest.TestCase):
    def setUp(self):
        self.test_input = """
            .#...#
            ......
            #.....
            ...#..
        """
        self.test_array = parse_input(self.test_input)

    def test_base_shape(self):
        self.assertEqual(self.test_array.shape, (4, 6))

        galaxy_locations = locate_galaxies(self.test_array)
        galaxy_locations_actual = np.array([[0, 1], [0, 5], [2, 0], [3, 3]])

        np.testing.assert_equal(galaxy_locations, galaxy_locations_actual)

    def expected_expansion(self, expansion: int) -> np.ndarray:
        return np.array(
            [
                [0, 1],
                [0, 5 + 2 * expansion],
                [2 + expansion, 0],
                [3 + expansion, 3 + expansion],
            ]
        )

    def test_expand_one(self):
        expansion = 1
        expanded_array = expand_array_gen(self.test_array, expansion)
        self.assertEqual(expanded_array.shape, (5, 8))

        galaxy_locations = locate_galaxies(expanded_array)
        galaxy_locations_actual = self.expected_expansion(expansion)

        np.testing.assert_equal(galaxy_locations, galaxy_locations_actual)

    def test_expand_two(self):
        expansion = 2
        expanded_array = expand_array_gen(self.test_array, expansion)
        self.assertEqual(expanded_array.shape, (6, 10))

        galaxy_locations = locate_galaxies(expanded_array)
        galaxy_locations_actual = self.expected_expansion(expansion)

        np.testing.assert_equal(galaxy_locations, galaxy_locations_actual)
