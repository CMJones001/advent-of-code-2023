# /usr/bin/env python
from typing import Optional

import numpy as np
from skimage import measure, morphology

test_input = """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
""".strip().split(
    "\n"
)


def problem_1():
    test_output = process_input(test_input)
    expected_output = 4361

    if test_output == expected_output:
        print(f"Test 1 output: {test_output}")
    else:
        print(f"Test 1 failed. Expected {expected_output}, got {test_output}")

    with open("problem_text") as f:
        input_ = f.readlines()

    input_ = [line.strip() for line in input_]
    problem_output = process_input(input_)
    print(f"Problem 1 output: {problem_output}")


def problem_2():
    test_output = process_gear_input(test_input)
    expected_output = 467835

    if test_output == expected_output:
        print(f"Test 2 output: {test_output}")
    else:
        print(f"Test 2 failed. Expected {expected_output}, got {test_output}")

    with open("problem_text") as f:
        input_ = f.readlines()

    input_ = [line.strip() for line in input_]
    problem_output = process_gear_input(input_)
    print(f"Problem 2 output: {problem_output}")


def process_input(input_: list[str]) -> int:
    """Process the input for problem 1."""
    number_array, symbol_mask = convert_to_arrays(input_)

    # The labelled image gives a different number for each connected number
    labelled_image = morphology.label(number_array >= 0)

    # Expand the symbol mask to include adjacent pixels
    expanded_mask = expand_mask(symbol_mask)

    # Get the regions that are adjacent to the original mask
    region_props = measure.regionprops(labelled_image, intensity_image=expanded_mask)
    regions_with_symbols = [
        region for region in region_props if region.max_intensity > 0
    ]

    total = 0
    for region in regions_with_symbols:
        slice_ = region.slice
        # Retrieve the numbers from the original input
        intensity_slice = number_array[slice_]
        num = _array_to_num(intensity_slice)
        total += num

    return total


def process_gear_input(input_: list[str]) -> int:
    """Process the input for problem 2."""
    # The labelled image gives a different number for each connected number
    test_array, _ = convert_to_arrays(input_)
    labelled_image = morphology.label(test_array >= 0)

    # Create a dictionary that maps the labelled_image number to the number in the original input
    number_regions = measure.regionprops(labelled_image, intensity_image=test_array)
    number_lookup = {
        region.label: _array_to_num(region.image_intensity) for region in number_regions
    }

    # Create a mask that numbers each (potential) gear/star
    gear_mask = get_gear_mask(input_)

    # Expand the gear mask to include adjacent pixels, keeping the numbering
    # This might break if there are gears that are adjacent to each other
    kernel = np.ones((3, 3))
    labelled_gears = morphology.dilation(gear_mask, kernel)

    # Look for gear regions that have two different labels in them
    # Then look these numbers up in the number_lookup dictionary to map back to the original input
    total = 0
    region_props = measure.regionprops(labelled_gears, intensity_image=labelled_image)
    for region in region_props:
        unique_values = is_gear_region(region)
        if unique_values is None:
            continue

        a, b = unique_values
        num_a = number_lookup[a]
        num_b = number_lookup[b]
        total += num_a * num_b

    return total


def is_gear_region(
    region: measure._regionprops.RegionProperties,
) -> Optional[tuple[int, int]]:
    """A gear region will have two different numbers in it, excluding zero."""
    image = region.image_intensity
    unique_values = np.unique(image[image != 0])

    if len(unique_values) == 2:
        return unique_values
    else:
        return None


def get_gear_mask(input_: list[str]) -> np.ndarray:
    """Uniquely label each gear in the input.

    Starts at 1 and increments for each gear.
    """
    gear_mask = []
    count = 0
    for line in input_:
        gear_mask_line = [(count := count + 1) if i == "*" else 0 for i in line]
        gear_mask.append(gear_mask_line)

    return np.array(gear_mask)


def convert_to_arrays(input_: list[str]):
    """
    Convert the input string into two arrays.

    The first array is the numbers, the second is a bool mask of the symbols.
    """
    number_array = []
    symbol_mask = []
    for line in input_:
        array_line = [_get_int_from_char(c) for c in line]
        mask_line = [_convert_symbol_to_int(c) for c in line]
        number_array.append(array_line)
        symbol_mask.append(mask_line)

    return np.array(number_array), np.array(symbol_mask)


def expand_mask(mask: np.ndarray) -> np.ndarray:
    foot_print = np.ones((3, 3))
    return morphology.binary_dilation(mask, foot_print)


def _get_int_from_char(char) -> int:
    """If char is a number, return the number, otherwise return -1"""
    try:
        return int(char)
    except ValueError:
        return -1


def _convert_symbol_to_int(symbol) -> int:
    """Return 1 if this is a misc symbol, 0 otherwise."""
    disallowed_symbols = [f"{i}" for i in range(10)] + ["."]
    return symbol not in disallowed_symbols


def _array_to_num(array: np.ndarray) -> int:
    """Convert a array of numbers into a single number by concatenation."""
    return int("".join([str(i) for i in array.ravel()]))


if __name__ == "__main__":
    problem_1()
    problem_2()
