use std::{collections::HashMap, env};

use asd::{
    gfa::{Entry, Orientation},
    parser,
};
use fdg::{
    fruchterman_reingold::{FruchtermanReingoldConfiguration, FruchtermanReingoldParallel},
    petgraph::Graph,
    // simple::Center,
    Force,
    ForceGraph,
};

use macroquad::prelude::*;

#[macroquad::main("fdg demo")]
async fn main() {
    let mut graph = Graph::<(String, Orientation), ()>::new();

    let file = std::fs::File::open(env::args().nth(1).expect("missing gfa file argument")).unwrap();
    let entries = parser::parse_source(file).unwrap();

    let mut index_map = HashMap::new();

    for entry in entries {
        println!("{:?}", entry);

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
                .or_insert_with(|| graph.add_node((from.clone(), from_orient)))
                .to_owned();

            // add second node if not present
            let b = index_map
                .entry(to.clone())
                .or_insert_with(|| graph.add_node((to.clone(), to_orient)))
                .to_owned();

            graph.add_edge(a, b, ());
        }
    }

    let mut force_graph: ForceGraph<f32, 2, (String, Orientation), ()> =
        fdg::init_force_graph_uniform(graph, 1000.0);

    // custom closure force which rotates each node
    // let mut rotate = |graph: &mut ForceGraph<f32, 2, (String, Orientation), ()>| {
    //     graph
    //         .node_weights_mut()
    //         .for_each(|(_, p)| *p = Rotation2::new(0.005).transform_point(p))
    // };
    let mut force = FruchtermanReingoldParallel {
        conf: FruchtermanReingoldConfiguration {
            scale: 100.0,
            ..Default::default()
        },
        ..Default::default()
    };

    loop {
        println!("frame");
        force.apply_many(&mut force_graph, 2);

        // Center::default().apply(&mut force_graph);
        // rotate.apply(&mut force_graph);

        let scale = calculate_scale(&force_graph);

        clear_background(WHITE);

        for idx in force_graph.edge_indices() {
            let ((_, source), (_, target)) = force_graph
                .edge_endpoints(idx)
                .map(|(a, b)| {
                    (
                        force_graph.node_weight(a).unwrap(),
                        force_graph.node_weight(b).unwrap(),
                    )
                })
                .unwrap();

            draw_line(
                translate_x(source.coords.column(0)[0], scale),
                translate_y(source.coords.column(0)[1], scale),
                translate_x(target.coords.column(0)[0], scale),
                translate_y(target.coords.column(0)[1], scale),
                1.0,
                BLACK,
            );
        }

        for ((_name, _orient), pos) in force_graph.node_weights() {
            let x = translate_x(pos.coords.column(0)[0], scale);
            let y = translate_y(pos.coords.column(0)[1], scale);

            draw_circle(x, y, 2.0, RED);
            // draw_text(
            //     format!("{}{}", name, orient).as_str(),
            //     x - 30.0 * scale,
            //     y - 30.0 * scale,
            //     40.0 * scale,
            //     BLACK,
            // );
        }

        next_frame().await
    }
}

fn translate_x(x: f32, scale: f32) -> f32 {
    (screen_width() / 2.0) + (x * scale)
}

fn translate_y(y: f32, scale: f32) -> f32 {
    (screen_height() / 2.0) + (y * scale)
}

fn calculate_scale(graph: &ForceGraph<f32, 2, (String, Orientation), ()>) -> f32 {
    let (min_x, max_x, min_y, max_y) = graph.node_weights().fold(
        (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
        |(min_x, max_x, min_y, max_y), (_, pos)| {
            (
                min_x.min(pos.coords.column(0)[0]),
                max_x.max(pos.coords.column(0)[0]),
                min_y.min(pos.coords.column(0)[1]),
                max_y.max(pos.coords.column(0)[1]),
            )
        },
    );

    let graph_width = max_x - min_x;
    let graph_height = max_y - min_y;

    let scale_x = screen_width() / graph_width;
    let scale_y = screen_height() / graph_height;

    scale_x.min(scale_y) * 0.9 // add some padding
}
