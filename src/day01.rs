pub fn solve(input: &str) -> u64 {
    let mut position = 50i32;
    let mut count = 0;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance = line[1..].parse::<i32>().unwrap();
        let sign = match direction {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid direction"),
        };
        let (new_position, zeros) = rotate(position, distance * sign);
        position = new_position;
        count += zeros;
    }
    count as u64
}

/// Given a start position in the 0-99 range and a movement,
/// Calculate the end position and the number of times zero was passed.
fn rotate(position: i32, movement: i32) -> (i32, u32) {
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

    pub const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE), 6);
    }

    /// Like the rotate implementation, but instead of using divisions and remainders,
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

    proptest! {

        #[test]
        fn test_rotate(position in 0..100, movement in -500..500) {
            prop_assert_eq!(rotate(position, movement), slow_rotate(position, movement));
        }
    }
}
