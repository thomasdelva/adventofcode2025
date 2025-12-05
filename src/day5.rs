use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use range_set::RangeSet;

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
    let mut range_set: RangeSet<[RangeInclusive<u64>; 8]> = RangeSet::new();
    for &(start, end) in ranges {
        range_set.insert_range(start..=end);
    }

    numbers.iter().filter(|&n| range_set.contains(*n)).count()
}

#[aoc(day5, part2)]
fn part2((ranges, _): &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    let mut range_set: RangeSet<[RangeInclusive<u64>; 8]> = RangeSet::new();
    for &(start, end) in ranges {
        range_set.insert_range(start..=end);
    }

    range_set.len()
}
