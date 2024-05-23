use std::collections::HashMap;

use asd::{
    gfa::{Entry, Orientation},
    parser,
};
use eframe::{run_native, App, CreationContext};
use egui::Context;
use egui_graphs::{
    DefaultEdgeShape, DefaultNodeShape, Graph, GraphView, SettingsInteraction, SettingsStyle,
};
use petgraph::stable_graph::StableGraph;

pub struct InteractiveApp {
    g: Graph<(String, Orientation), ()>,
}

impl InteractiveApp {
    fn new(_: &CreationContext<'_>) -> Self {
        let g = generate_graph();
        Self { g }
    }
}

impl App for InteractiveApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let interaction_settings = &SettingsInteraction::new()
                .with_dragging_enabled(true)
                .with_node_clicking_enabled(true)
                .with_node_selection_enabled(true)
                .with_node_selection_multi_enabled(true)
                .with_edge_clicking_enabled(true)
                .with_edge_selection_enabled(true)
                .with_edge_selection_multi_enabled(true);
            let style_settings = &SettingsStyle::new().with_labels_always(true);
            ui.add(
                &mut GraphView::<_, _, _, _, DefaultNodeShape, DefaultEdgeShape>::new(&mut self.g)
                    .with_styles(style_settings)
                    .with_interactions(interaction_settings),
            );
        });
    }
}

fn generate_graph() -> Graph<(String, Orientation), ()> {
    let mut g: StableGraph<(String, Orientation), ()> = StableGraph::new();

    let file = std::fs::File::open("../../dataset/example.gfa").unwrap();
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
                .or_insert_with(|| g.add_node((from.clone(), from_orient)))
                .to_owned();

            // add second node if not present
            let b = index_map
                .entry(to.clone())
                .or_insert_with(|| g.add_node((to.clone(), to_orient)))
                .to_owned();

            g.add_edge(a, b, ());
        }
    }

    Graph::from(&g)
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "egui_graphs_interactive_demo",
        native_options,
        Box::new(|cc| Box::new(InteractiveApp::new(cc))),
    )
    .unwrap();
}
