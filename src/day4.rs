use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

/// We pad the input grid on all four sides with empty values.
///
/// This makes computing the neighbours easier: moving 1 cell in any direction
/// from any cell in the unpadded grid will be within bounds of the padded grid.
///
/// This removes the need for a "is the neighbour within bounds" check
/// from the tight inner loop (this is an example of branchless programming).
/// This empirically improved performance by ~20%.
#[aoc_generator(day4)]
fn rolls_and_width(input: &str) -> (Vec<bool>, usize) {
    let width = input.lines().next().unwrap().len() + 2;
    let mut padded_rolls = vec![false; width];
    padded_rolls.extend(input.lines().flat_map(|line| {
        let mut padded_line = Vec::with_capacity(line.len() + 2);
        padded_line.push(false);
        padded_line.extend(line.chars().map(|char| char == '@'));
        padded_line.push(false);
        padded_line
    }));
    padded_rolls.extend(vec![false; width]);
    (padded_rolls, width)
}

#[aoc(day4, part1)]
fn part1((rolls, width): &(Vec<bool>, usize)) -> usize {
    neighbour_count(rolls, *width)
        .iter()
        .enumerate()
        .filter(|&(i, _)| rolls[i])
        .filter(|&(_, count)| *count <= 4)
        .count()
}

/// Counts the amount of removed rolls by counting the neighbouring rolls
/// of each cell and updating those counts when removing rolls.
/// When a roll's neighbour count is updated to a value below the threshold,
/// that roll is also marked for removal.
#[aoc(day4, part2)]
fn part2((rolls, width): &(Vec<bool>, usize)) -> usize {
    let mut neighbour_count = neighbour_count(rolls, *width);

    let mut to_remove: VecDeque<usize> = neighbour_count
        .iter()
        .enumerate()
        .filter(|&(i, _)| rolls[i])
        .filter(|&(_, count)| *count <= 4)
        .map(|(i, _)| i)
        .collect();

    let mut total_removed = 0;
    while let Some(roll) = to_remove.pop_front() {
        total_removed += 1;

        // Update this rolls neighbour counts and enqueue any new rolls for removal.
        for_neighbours(roll, *width, &mut |neighbour| {
            neighbour_count[neighbour] -= 1;
            if rolls[neighbour] && neighbour_count[neighbour] == 4 {
                to_remove.push_back(neighbour);
            }
        });
    }

    total_removed
}

/// For each position in the grid, including the padded positions,
/// calculate the amount of neighbours.
///
/// Beware of a slight quirk: the neighbour count of a roll includes itself.
/// This makes counting faster, another example of branchless programming.
///
/// We're a bit lucky that the padded cells have <=3 neighbouring rolls,
/// safely below the cutoff of 4 from the problem statement.
fn neighbour_count(rolls: &Vec<bool>, width: usize) -> Vec<u8> {
    let height = rolls.len() / width;
    let mut neighbour_count = vec![0; rolls.len()];
    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            let roll = row * width + col;
            if rolls[roll] {
                for_neighbours(roll, width, &mut |neighbour| {
                    neighbour_count[neighbour] += 1;
                });
            }
        }
    }
    neighbour_count
}

/// Apply the function on the given roll and its direct and diagonal neighbours.
///
/// This is about equivalent to `for n in neighbours(roll, width) { f(n) }`,
/// but I found that returning the neighbours was a bit (~20%) slower,
/// even when returning an iterator using `itertools::cartesian_product`.
fn for_neighbours(roll: usize, width: usize, f: &mut dyn FnMut(usize)) {
    let row = roll / width;
    let col = roll % width;

    for r in (row - 1)..=(row + 1) {
        for c in (col - 1)..=(col + 1) {
            f(r * width + c);
        }
    }
}
