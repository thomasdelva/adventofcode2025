use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_locations(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

#[aoc(day9, part1)]
fn largest_rectangle_area(locations: &[(usize, usize)]) -> usize {
    let mut max_area = 0;
    for (i, &(x1, y1)) in locations.iter().enumerate() {
        for &(x2, y2) in &locations[i + 1..] {
            max_area = max_area.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }
    max_area
}

#[aoc(day9, part2)]
fn largest_colored_area(locations: &[(usize, usize)]) -> usize {
    // Even though there are many (~100k) rows in the input,
    // only a small fractions has something interesting happening.
    //
    // We identify this fraction and sort it to make it accessible by binary search.
    let mut rows = locations.iter().map(|&(r, _)| r).collect::<Vec<_>>();
    rows.sort();
    rows.dedup();

    // For each row, we identify the columns where the loop crosses it.
    // This will help us identify which parts are inside the loop.
    let mut row_crossings = rows.iter().map(|_| Vec::new()).collect::<Vec<_>>();
    for ((r1, c1), (r2, c2)) in locations.iter().zip(locations.iter().cycle().skip(1)) {
        if c1 != c2 {
            // We're establishing vertical crossings, so ignore non-vertical edges
            continue;
        }
        let (r_min, r_max) = (r1.min(r2), r1.max(r2));
        for col in index(*r_min, &rows)..index(*r_max, &rows) {
            row_crossings[col].push(*c1);
        }
    }

    // Identify the internal parts per each row.
    //
    // Strictly speaking we're not identifying the internal parts of the rows themselves,
    // but rather the internal parts between each row and the next row.
    // That's a lot easier to do because we don't have to worry about horizontal edges
    // in those parts, they don't have any by definition.
    // Can we safely ignore the rows themselves? Yes, as long as there are never two
    // directly adjacent rows, without any other row between them.
    let internal_col_ranges_per_row: Vec<_> = row_crossings
        .into_iter()
        .map(|cs| internal_ranges(cs))
        .collect();

    let mut max_area = 0;
    for (i, (r1, c1)) in locations.iter().enumerate() {
        'rectangles: for (r2, c2) in locations[i + 1..].iter() {
            let (r_min, r_max) = (r1.min(r2), r1.max(r2));
            let (c_min, c_max) = (c1.min(c2), c1.max(c2));

            let start_row_index = index(*r_min, &rows);
            let end_row_index = index(*r_max, &rows);

            // A rectangle is completely internal if all its rows are covered by an internal range.
            // (There's an assumption here that there are never two touching column ranges.
            // This doesn't occur in the input, but we could handle it by coalescing touching ranges.)
            //
            // The same note as above applies here: we're not actually checking
            // whether the rows are internal, but rather the parts between the rows.
            for row_ranges in internal_col_ranges_per_row[start_row_index..end_row_index].iter() {
                if row_ranges
                    .iter()
                    .all(|r| r.start() > c_min || r.end() < c_max)
                {
                    continue 'rectangles;
                }
            }
            max_area = max_area.max((r1.abs_diff(*r2) + 1) * (c1.abs_diff(*c2) + 1));
        }
    }

    max_area
}

/// Each part between two crossings is internal to the loop.
fn internal_ranges(mut crossings: Vec<usize>) -> Vec<RangeInclusive<usize>> {
    crossings.sort();
    crossings.dedup();
    crossings.chunks(2).map(|w| w[0]..=w[1]).collect()
}

/// Finds the index of the value in the array.
fn index(value: usize, sorted_values: &[usize]) -> usize {
    sorted_values.binary_search(&value).unwrap()
}
