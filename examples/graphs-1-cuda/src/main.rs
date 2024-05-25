use cust::prelude::*;
use std::fs::File;
use std::io::Read;

async fn main() {
    // Initialize CUDA context
    let _ctx = cust::quick_init().unwrap();

    // Load the CUDA module
    let mut file = File::open("kernel.ptx").unwrap();
    let mut ptx = String::new();
    file.read_to_string(&mut ptx).unwrap();
    let module = Module::load_from_string(&ptx).unwrap();
    let stream = Stream::new(StreamFlags::DEFAULT, None).unwrap();

    let mut graph = load_graph();

    let mut desired_distance_matrix = HashMap::new();
    graph.node_indices().for_each(|idx| {
        desired_distance_matrix.insert(idx, dijkstra(&graph, idx, None, |_| 1.0 as f32));
    });

    loop {
        let num_nodes = graph.node_count();
        let max_neighbors = 10; // Adjust based on your data

        // Prepare data for GPU
        let positions_x: Vec<f32> = graph.node_weights().map(|(_, pos)| pos.x).collect();
        let positions_y: Vec<f32> = graph.node_weights().map(|(_, pos)| pos.y).collect();
        let mut forces_x = vec![0.0f32; num_nodes];
        let mut forces_y = vec![0.0f32; num_nodes];
        let neighbors = vec![-1; num_nodes * max_neighbors]; // Placeholder for neighbors
        let distances = vec![0.0f32; num_nodes * max_neighbors]; // Placeholder for distances

        // TODO: Fill neighbors and distances arrays based on your graph structure

        // Allocate device memory
        let positions_x_device = positions_x.as_slice().as_dvec().unwrap();
        let positions_y_device = positions_y.as_slice().as_dvec().unwrap();
        let forces_x_device = forces_x.as_mut_slice().as_dvec().unwrap();
        let forces_y_device = forces_y.as_mut_slice().as_dvec().unwrap();
        let neighbors_device = neighbors.as_slice().as_dvec().unwrap();
        let distances_device = distances.as_slice().as_dvec().unwrap();

        // Launch the CUDA kernel
        unsafe {
            launch!(module.compute_forces<<<num_nodes / 256 + 1, 256, 0, stream>>>(
                positions_x_device.as_device_ptr(),
                positions_y_device.as_device_ptr(),
                forces_x_device.as_device_ptr(),
                forces_y_device.as_device_ptr(),
                neighbors_device.as_device_ptr(),
                distances_device.as_device_ptr(),
                num_nodes as i32,
                max_neighbors as i32
            ))
            .unwrap();
        }

        stream.synchronize().unwrap();

        // Copy results back to host
        forces_x_device.copy_to(&mut forces_x).unwrap();
        forces_y_device.copy_to(&mut forces_y).unwrap();

        // Update node positions
        for (i, (force_x, force_y)) in forces_x.iter().zip(forces_y.iter()).enumerate() {
            let (_, pos) = graph.node_weight_mut(NodeIndex::new(i)).unwrap();
            pos.x += force_x;
            pos.y += force_y;
        }

        // Render
        let now = Instant::now();
        clear_background(WHITE);
        draw_graph(&graph);
        let elapsed = now.elapsed();
        println!("frame: {:?}", elapsed);
        next_frame().await;
    }
}

fn load_graph() -> StableGraph<(String, Point2<f32>), ()> {
    println!("Loading graph");

    let mut graph = StableGraph::new();

    let file = std::fs::File::open(env::args().nth(1).expect("missing gfa file argument")).unwrap();
    let entries = parser::parse_source(file).unwrap();

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
