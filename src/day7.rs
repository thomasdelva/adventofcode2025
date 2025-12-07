use aoc_runner_derive::{aoc, aoc_generator};

/// Parse the start position and the splitter positions as bit vectors.
#[aoc_generator(day7)]
fn parse_start_and_splitters(input: &str) -> (usize, Vec<Vec<bool>>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().find('S').unwrap();
    let splitters = lines
        .skip(1)
        .step_by(2)
        .map(|line| line.chars().map(|c| c == '^').collect())
        .collect();
    (start, splitters)
}

/// Count the number of beam splits using a bit vector of beams and updating it per row.
#[aoc(day7, part1)]
fn count_splits((start, splitters): &(usize, Vec<Vec<bool>>)) -> usize {
    let mut splits = 0;
    let mut beams: Vec<bool> = vec![false; splitters[0].len()];
    beams[*start] = true;
    for row_splitters in splitters {
        let mut new_beams = vec![false; beams.len()];
        for i in 0..beams.len() {
            if !beams[i] {
                continue;
            }
            if row_splitters[i] {
                new_beams[i - 1] = true;
                new_beams[i + 1] = true;
                splits += 1;
            } else {
                new_beams[i] = true;
            }
        }
        beams = new_beams;
    }
    splits
}

/// Count the number of beam paths using a vector of beam counts per position and updating it per row.
#[aoc(day7, part2)]
fn count_beam_paths((start, splitters): &(usize, Vec<Vec<bool>>)) -> usize {
    let mut beams: Vec<usize> = vec![0; splitters[0].len()];
    beams[*start] = 1;
    for row_splitters in splitters {
        let mut new_beams = vec![0; beams.len()];
        for i in 0..beams.len() {
            if beams[i] == 0 {
                continue;
            }
            if row_splitters[i] {
                new_beams[i - 1] += beams[i];
                new_beams[i + 1] += beams[i];
            } else {
                new_beams[i] += beams[i];
            }
        }
        beams = new_beams;
    }
    beams.iter().sum()
}
