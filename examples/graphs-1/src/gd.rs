#![allow(dead_code)]

use std::collections::HashMap;

pub struct Graph {
    edges_from: Vec<u32>,
    edges_to: Vec<u32>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges_from: Vec::new(),
            edges_to: Vec::new(),
        }
    }

    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            edges_from: Vec::with_capacity(capacity),
            edges_to: Vec::with_capacity(capacity),
        }
    }

    fn add_edge(&mut self, from: u32, to: u32) {
        self.edges_from.push(from);
        self.edges_to.push(to);
    }
}

pub fn update(
    _graph: &Graph,
    xs: &Vec<f32>,
    ys: &Vec<f32>,
    desired_distance_matrix: &HashMap<usize, HashMap<usize, f32>>,
) -> f32 {
    desired_distance_matrix
        .iter()
        .flat_map(|(&i, targets)| {
            targets
                .iter()
                .map(move |(&j, &target_distance)| (i, j, target_distance))
        })
        .map(|(i, j, target_distance)| {
            let dx = xs[j] - xs[i];
            let dy = ys[j] - ys[i];
            let distance_sqrd = dx * dx + dy * dy;
            let error = distance_sqrd - target_distance * target_distance;

            error * error
        })
        .sum()
}
