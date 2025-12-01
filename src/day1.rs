use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let distance = line[1..].parse::<i32>().unwrap();
            match direction {
                'L' => -distance,
                'R' => distance,
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn count_ended_zeroes(input: &Vec<i32>) -> u64 {
    let mut position = 50i32;
    let mut count = 0;
    for movement in input {
        position = (position + movement).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }
    count as u64
}

#[aoc(day1, part2)]
pub fn count_passed_zeroes(input: &Vec<i32>) -> u64 {
    let mut position = 50i32;
    let mut count = 0;
    for movement in input {
        let (new_position, zeros) = rotate(position, *movement);
        position = new_position;
        count += zeros;
    }
    count as u64
}

/// Given a start position in the 0-99 range and a movement,
/// Calculate the end position and the number of times zero was passed.
fn rotate(position: i32, movement: i32) -> (i32, u32) {
    let full_rotations = (movement / 100).abs() as u32;
    let end_position = (position + movement).rem_euclid(100);

    let extra_zero = if crosses_zero(position, movement % 100) {
        1
    } else {
        0
    };

    (end_position, full_rotations + extra_zero)
}

fn crosses_zero(position: i32, movement: i32) -> bool {
    if position == 0 {
        false
    } else {
        position + movement >= 100 || position + movement <= 0
    }
}

#[aoc(day1, part2, stepping)]
pub fn count_passed_zeroes_slow(input: &Vec<i32>) -> u64 {
    let mut position = 50i32;
    let mut count = 0;
    for movement in input {
        let (new_position, zeros) = slow_rotate(position, *movement);
        position = new_position;
        count += zeros;
    }
    count as u64
}

/// Like rotate above, but instead of using divisions and remainders,
/// it takes small steps and counts the zeroes.
fn slow_rotate(position: i32, movement: i32) -> (i32, u32) {
    let step = movement.signum();
    let mut zeros = 0;
    let mut position = position;

    for _ in 0..movement.abs() {
        position = (position + step).rem_euclid(100);
        if position == 0 {
            zeros += 1;
        }
    }
    (position, zeros)
}

// TESTS

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, proptest};

    use super::*;

    proptest! {
        #[test]
        fn test_rotate(position in 0..100, movement in -500..500) {
            prop_assert_eq!(rotate(position, movement), slow_rotate(position, movement));
        }
    }
}
