use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct XYZ {
    x: u32,
    y: u32,
    z: u32,
}

/// Note: sorts on distance first, then breaks ties using the points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    square_distance: u64,
    start_index: usize,
    end_index: usize,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<XYZ> {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.split(',').map(|part| part.parse().unwrap()).collect();
            XYZ {
                x: numbers[0],
                y: numbers[1],
                z: numbers[2],
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn sum_of_connected_components(points: &[XYZ]) -> u32 {
    // Default is a max heap, while we need a min heap
    let mut edges = Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (start, end) = (points[i], points[j]);
            let sq_dist = (start.x as u64 - end.x as u64).pow(2)
                + (start.y as u64 - end.y as u64).pow(2)
                + (start.z as u64 - end.z as u64).pow(2);
            edges.push(Edge {
                square_distance: sq_dist,
                start_index: i,
                end_index: j,
            });
        }
    }

    let (shortest_edges, _, _) = edges.select_nth_unstable(1001);

    let mut disjoint_sets: Vec<usize> = (0..points.len()).collect();
    for edge in shortest_edges {
        let (i, j) = (edge.start_index, edge.end_index);
        let (i_rep, j_rep) = (find(i, &mut disjoint_sets), find(j, &mut disjoint_sets));
        if i_rep != j_rep {
            disjoint_sets[i_rep] = j_rep;
        }
    }

    let mut component_sizes = vec![0; points.len()];
    for i in 0..points.len() {
        component_sizes[find(i, &mut disjoint_sets)] += 1;
    }
    component_sizes.sort_unstable();
    component_sizes.iter().rev().take(3).product()
}

#[aoc(day8, part2)]
fn x_product_of_last_merge(points: &[XYZ]) -> u64 {
    // Default is a max heap, while we need a min heap
    let mut edges: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();

    // Initialize the heap with all edges
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (start, end) = (points[i], points[j]);
            let sq_dist = (start.x as u64 - end.x as u64).pow(2)
                + (start.y as u64 - end.y as u64).pow(2)
                + (start.z as u64 - end.z as u64).pow(2);
            edges.push(Reverse(Edge {
                square_distance: sq_dist,
                start_index: i,
                end_index: j,
            }));
        }
    }

    // Keep adding the cheapest edge until we only have one circuit left
    let mut disjoint_sets: Vec<usize> = (0..points.len()).collect();
    let mut num_circuits = points.len();
    let last_edge = loop {
        let Reverse(edge) = edges.pop().unwrap();
        let (i, j) = (edge.start_index, edge.end_index);
        let (i_rep, j_rep) = (find(i, &mut disjoint_sets), find(j, &mut disjoint_sets));
        if i_rep != j_rep {
            disjoint_sets[i_rep] = j_rep;
            num_circuits -= 1;
            if num_circuits == 1 {
                break edge;
            }
        }
    };
    points[last_edge.start_index].x as u64 * points[last_edge.end_index].x as u64
}

/// If i is part of a set, returns the set representative and compresses the path to it.
/// If i is not part of a set, adds a new singleton set and returns its representative, i.
fn find(i: usize, disjoint_sets: &mut Vec<usize>) -> usize {
    let parent = disjoint_sets[i];
    if parent == i {
        i
    } else {
        let root = find(parent, disjoint_sets);
        disjoint_sets[i] = root;
        root
    }
}
