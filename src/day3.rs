use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 48).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn count_joltage_2(input: &Vec<Vec<u8>>) -> u64 {
    input.iter().map(|line| joltage_n(line, 2)).sum()
}

#[aoc(day3, part2)]
pub fn count_joltage_12(input: &Vec<Vec<u8>>) -> u64 {
    input.iter().map(|line| joltage_n(line, 12)).sum()
}

/// Calculates the highest n-digit number that can be made with digits in the slice.
fn joltage_n(line: &[u8], n: usize) -> u64 {
    // Recursion base case
    if n == 0 {
        return 0;
    }

    // Find the position of the highest digit that still has n-1 digits behind it,
    // using the earliest occurence in case there are multiple highest digits.
    // This digit is the only option we need to consider in this iteration because
    // any later or lower digit would lead to worse (or perhaps equal) results.
    let position = argmax(&line[0..line.len() - n + 1]);

    // Recursively calculate the (n-1)-digit number that can be made with the remaining digits.
    let joltage_n_minus_one = joltage_n(&line[position + 1..], n - 1);

    // Add the digit at `position` in front of the (n-1)-digit number.
    10u64.pow(n as u32 - 1) * line[position] as u64 + joltage_n_minus_one
}

/// Finds the position of the first, highest value in the slice.
/// If the slice is empty, 0 is returned.
fn argmax(digits: &[u8]) -> usize {
    let mut max = 0u8;
    let mut pos = 0;
    for (i, &value) in digits.iter().enumerate() {
        if value == 9 {
            // Early return because we know the 9 is the highest digit
            return i;
        }
        if value > max {
            max = value;
            pos = i;
        }
    }
    pos
}
