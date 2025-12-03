use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(u64, u64)> {
    let line = input.lines().next().unwrap();
    line.split(",")
        .map(|s| {
            let mut pair = s.split("-");
            (
                pair.next().unwrap().parse().unwrap(),
                pair.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .flat_map(|&(start, end)| split_on_nr_digits(start, end))
        .map(|(start, end)| sum_of_n_repeats(start, end, 2))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .flat_map(|&(start, end)| split_on_nr_digits(start, end))
        .map(|(start, end)| sum_of_repeats(start, end))
        .sum()
}

/// Sums all numbers with a repetitive pattern.
fn sum_of_repeats(start: u64, end: u64) -> u64 {
    // First we gather the sums per frequency, knowing we are double-counting some sums.
    // For example, a number like 111111 has a 2,3 and 6-repeating pattern
    // and will show up in three sums.
    let repeats_by_n: Vec<u64> = (2..=nr_digits(start))
        .map(|n| sum_of_n_repeats(start, end, n))
        .collect();

    // Sum the repetitions, while subtracting the double-counted ones.
    let mut total = 0;
    for n in 2..=nr_digits(start) {
        total += repeats_by_n[n as usize - 2];
        for i in 2..n {
            if n % i == 0 {
                total -= repeats_by_n[n as usize - 2];
            }
        }
    }
    total
}

/// Gives the sum of all n-repeating numbers in the [start, end] range.
///
/// Assumes start and end have the same number of digits,
/// this can be guaranteed by split_on_nr_digits.
fn sum_of_n_repeats(start: u64, end: u64, n: u32) -> u64 {
    let Some(divisor) = divisor(nr_digits(start), n) else {
        return 0;
    };

    let lowest_quotient = start.div_ceil(divisor);
    let highest_quotient = end / divisor;

    if lowest_quotient > highest_quotient {
        return 0;
    }

    (lowest_quotient + highest_quotient) * (highest_quotient - lowest_quotient + 1) / 2 * divisor
}

/// Returns a divisor to check if a number of length nr_digits repeats a pattern nr_repeats times.
///
/// For example, length-6 123123 repeats 123 2 times and is divisible by divisor(6, 2) = 1001.
fn divisor(nr_digits: u32, nr_repeats: u32) -> Option<u64> {
    if nr_digits % nr_repeats != 0 {
        // Can only repeat a pattern if the number of digits is a multiple of the pattern length
        return None;
    }

    let mut divisor = 1u64;
    for _ in 1..nr_repeats {
        divisor *= 10u64.pow(nr_digits / nr_repeats);
        divisor += 1;
    }
    Some(divisor)
}

/// Splits the range into subranges that have a constant number of digits.
/// For example, the range [90, 110] would be split into [90, 99] and [100, 110],
/// i.e., the subranges where numbers have 2 and 3 digits.
fn split_on_nr_digits(start: u64, end: u64) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();
    let mut current_start = start;
    for i in nr_digits(start)..nr_digits(end) {
        ranges.push((current_start, 10u64.pow(i) - 1));
        current_start = 10u64.pow(i);
    }
    ranges.push((current_start, end));

    ranges
}

fn nr_digits(i: u64) -> u32 {
    i.ilog10() + 1
}

// TESTS

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, proptest};

    use super::*;

    #[test]
    fn divisor_examples() {
        assert_eq!(divisor(10, 5), Some(101010101));
        assert_eq!(divisor(6, 2), Some(1001));
    }

    proptest! {
        #[test]
        fn test_split_on_nr_digits(a in 1..u64::MAX, b in 1..u64::MAX) {
            let start = a.min(b);
            let end = a.max(b);

            let ranges = split_on_nr_digits(start, end);

            // Subranges are continuous
            for (&(_, end_1), &(start_2, _)) in ranges.iter().zip(ranges.iter().skip(1)) {
                prop_assert_eq!(end_1 + 1, start_2)
            }

            // Subranges have constant digit length
            for &(s, e) in ranges.iter() {
                prop_assert_eq!(nr_digits(s), nr_digits(e));
            }

            // Subranges cover the initial range completely
            let &(s, _) = ranges.first().unwrap();
            let &(_, e) = ranges.last().unwrap();
            prop_assert_eq!(s, start);
            prop_assert_eq!(e, end);
        }

        #[test]
        fn test_nr_digits(i in 0..u64::MAX) {
            prop_assert_eq!(nr_digits(i), i.to_string().len() as u32);
        }
    }
}
