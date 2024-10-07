use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader},
    ops::AddAssign,
    time::Instant,
};

use asd::{gfa::Entry, parser};
use indicatif::ProgressIterator;
use macroquad::{prelude::*, rand, ui::root_ui};
use nalgebra::{Point2, SVector};
use petgraph::{algo::dijkstra, graph::NodeIndex, stable_graph::StableGraph};

use rayon::prelude::*;

mod gd;

#[macroquad::main("graphs_1")]
async fn main() {
    println!("Hello, world!");

    let mut graph = load_graph();

    let mut desired_distance_matrix = HashMap::new();

    graph.node_indices().for_each(|idx| {
        desired_distance_matrix.insert(idx, dijkstra(&graph, idx, None, |_| 1.0 as f32));
    });

    // println!("{:?}", desired_distance_matrix);

    loop {
        // Update

        let start_update = Instant::now();

        update(&mut graph, &desired_distance_matrix);

        let update_elapsed = start_update.elapsed();

        // Render

        let start_render = Instant::now();

        clear_background(WHITE);

        draw_graph(&graph);

        let render_elapsed = start_render.elapsed();

        root_ui().label(
            None,
            format!("update: {:?}, render: {:?}", update_elapsed, render_elapsed).as_str(),
        );

        // println!("update: {:?}, render: {:?}", update_elapsed, render_elapsed);

        next_frame().await
    }
}

fn update(
    graph: &mut StableGraph<(String, Point2<f32>), ()>,
    desired_distance_matrix: &HashMap<NodeIndex, HashMap<NodeIndex, f32>>,
) {
    for _ in 1..2 {
        let forces = graph
            .node_indices()
            .par_bridge()
            .map(|idx| {
                (
                    idx,
                    desired_distance_matrix
                        .get(&idx)
                        .unwrap()
                        .par_iter()
                        .map(|(&other_idx, &distance)| {
                            let pos = graph.node_weight(idx).unwrap().1;
                            let other_pos = graph.node_weight(other_idx).unwrap().1;
                            let delta = other_pos - pos;
                            let dist = delta.norm_squared();
                            let correction = dist - (distance * distance);

                            // println!("correction: {:?}", correction);

                            if distance > 0.0 && dist > 1e-6 {
                                0.01 * delta.normalize() * correction.atan()
                            } else {
                                SVector::<f32, 2>::zeros()
                            }
                        })
                        .sum::<SVector<f32, 2>>()
                        + {
                            let pos = graph.node_weight(idx).unwrap().1;

                            let delta = pos - Point2::new(0.0, 0.0);

                            let dist = delta.norm_squared();

                            if dist > 1e-6 {
                                -0.01 * delta.normalize() / dist.max(1.0)
                            } else {
                                SVector::<f32, 2>::zeros()
                            }
                        }
                        + graph
                            .node_indices()
                            .par_bridge()
                            .filter(|&other_idx| other_idx != idx)
                            .map(|other_idx| {
                                let pos = graph.node_weight(idx).unwrap().1;
                                let other_pos = graph.node_weight(other_idx).unwrap().1;
                                let delta = other_pos - pos;
                                let dist = delta.norm();
                                if dist > 1e-3 {
                                    -0.001 * delta.normalize() / (dist * dist)
                                } else {
                                    SVector::<f32, 2>::zeros()
                                }
                            })
                            .sum::<SVector<f32, 2>>(),
                )
            })
            .collect::<HashMap<_, _>>();

        forces.iter().for_each(|(idx, force)| {
            let (_, pos) = graph.node_weight_mut(*idx).unwrap();
            pos.add_assign(force);
        });
    }
}

fn load_graph() -> StableGraph<(String, Point2<f32>), ()> {
    println!("Loading graph");

    let mut graph = StableGraph::new();

    let filename = env::args().nth(1).expect("missing gfa file argument");
    let entries = parser::parse_file(filename).expect("failed to parse file");

    let mut index_map = HashMap::new();

    let node_count = entries
        .iter()
        .filter_map(|entry| match entry {
            Entry::Segment { id, .. } => Some(id),
            _ => None,
        })
        .count();

    println!("Node count: {}", node_count);

    let radius = (node_count as f32).sqrt();

    let mut i = -10.0;

    for entry in entries
        .iter()
        .filter(|entry| matches!(entry, Entry::Link { .. }))
        .take(3000)
    {
        // println!("{:?}", entry);

        if let Entry::Link {
            from,
            from_orient,
            to,
            to_orient,
        } = entry
        {
            // add first node if not present
            let a = index_map
                .entry(from.clone())
                .or_insert_with(|| {
                    i += 1.0;

                    graph.add_node((
                        format!("{}{}", from, from_orient),
                        Point2::new(rand::gen_range(0.0, radius), rand::gen_range(0.0, radius)),
                        // Point2::new(i, 50.0 + rand::gen_range(0.0, 100.0)),
                    ))
                })
                .to_owned();

            // add second node if not present
            let b = index_map
                .entry(to.clone())
                .or_insert_with(|| {
                    i += 1.0;

                    graph.add_node((
                        format!("{}{}", from, to_orient),
                        Point2::new(rand::gen_range(0.0, radius), rand::gen_range(0.0, radius)),
                        // Point2::new(i, 50.0 + rand::gen_range(0.0, 100.0)),
                    ))
                })
                .to_owned();

            graph.add_edge(a, b, ());
            graph.add_edge(b, a, ());
        }
    }

    println!("Loading completed");

    graph
}

fn draw_graph(graph: &StableGraph<(String, Point2<f32>), ()>) {
    let (width, height) = (screen_width(), screen_height());

    let (min_x, max_x) = graph
        .node_weights()
        .map(|(_, pos)| pos.x)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), x| {
            (min.min(x), max.max(x))
        });

    let (min_y, max_y) = graph
        .node_weights()
        .map(|(_, pos)| pos.y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), y| {
            (min.min(y), max.max(y))
        });

    let source_range: f32 = (max_x - min_x).max(max_y - min_y);

    for idx in graph.edge_indices() {
        let ((_, source), (_, target)) = graph
            .edge_endpoints(idx)
            .map(|(a, b)| (graph.node_weight(a).unwrap(), graph.node_weight(b).unwrap()))
            .unwrap();

        draw_line(
            remap(source.x, min_x, min_x + source_range, 10.0, width - 10.0),
            remap(source.y, min_y, min_y + source_range, 10.0, height - 10.0),
            remap(target.x, min_x, min_x + source_range, 10.0, width - 10.0),
            remap(target.y, min_y, min_y + source_range, 10.0, height - 10.0),
            1.0,
            BLACK,
        );
    }

    for (_label, pos) in graph.node_weights() {
        let x = remap(pos.x, min_x, min_x + source_range, 10.0, width - 10.0);
        let y = remap(pos.y, min_y, min_y + source_range, 10.0, height - 10.0);

        draw_circle(x, y, 2.0, RED);
        // draw_text(label.as_str(), x - 30.0, y - 30.0, 10.0, BLACK);
    }

    draw_line(
        remap(0.0, min_x, min_x + source_range, 10.0, width - 10.0),
        remap(0.0, min_y, min_y + source_range, 10.0, height - 10.0),
        remap(100.0, min_x, min_x + source_range, 10.0, width - 10.0),
        remap(0.0, min_y, min_y + source_range, 10.0, height - 10.0),
        2.0,
        BLUE,
    );

    draw_line(
        remap(0.0, min_x, min_x + source_range, 10.0, width - 10.0),
        remap(0.0, min_y, min_y + source_range, 10.0, height - 10.0),
        remap(0.0, min_x, min_x + source_range, 10.0, width - 10.0),
        remap(100.0, min_y, min_y + source_range, 10.0, height - 10.0),
        2.0,
        BLUE,
    );
}

fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}
