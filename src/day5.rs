use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut lines = input.lines();

    let mut ranges = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        ranges.push((start, end));
    }

    let mut numbers = Vec::new();
    while let Some(line) = lines.next() {
        numbers.push(line.parse().unwrap());
    }

    (ranges, numbers)
}

#[aoc(day5, part1)]
fn part1((ranges, numbers): &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    let sorted_disjoint_ranges = sorted_disjoint_ranges(ranges);

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let mut count = 0;

    // Iterate over both the sorted ranges and the sorted numbers at the same time.
    let mut rangerator = sorted_disjoint_ranges.iter().peekable();
    let mut numberator = sorted_numbers.iter().peekable();
    while let (Some((start, end)), Some(&number)) = (rangerator.peek(), numberator.peek()) {
        // Advance either the number or range iterator, whichever is behind.
        if number < start {
            numberator.next();
        } else if number > end {
            rangerator.next();
        } else {
            count += 1;
            numberator.next();
        }
    }

    count
}

#[aoc(day5, part2)]
fn part2((ranges, _): &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    sorted_disjoint_ranges(ranges)
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum()
}

fn sorted_disjoint_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    let mut sorted_disjoint_ranges = Vec::new();
    let mut active_range: Option<(u64, u64)> = None;
    for (start, end) in sorted_ranges {
        match active_range {
            Some((active_start, active_end)) if start > active_end + 1 => {
                sorted_disjoint_ranges.push((active_start, active_end));
                active_range = Some((start, end));
            }
            Some((active_start, active_end)) => {
                active_range = Some((active_start, u64::max(active_end, end)));
            }
            None => active_range = Some((start, end)),
        }
    }

    if let Some((start, end)) = active_range {
        sorted_disjoint_ranges.push((start, end));
    }
    sorted_disjoint_ranges
}
