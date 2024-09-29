use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hash},
    iter::Sum,
    ops::AddAssign,
};

// use nalgebra::{Point, SVector};
// use petgraph::stable_graph::NodeIndex;

// type HashFn = BuildHasherDefault<rustc_hash::FxHasher>;

use fdg::Field;
use nalgebra::Point;

#[derive(Debug, Clone)]
pub struct StressMajorizationConfiguration<F: Field> {
    pub dt: F,
}

use petgraph::{
    algo::{dijkstra, Measure},
    graph::NodeIndex,
    stable_graph::StableGraph,
};
use StressMajorizationConfiguration as Config;

impl<F: Field> Default for Config<F> {
    fn default() -> Self {
        Self {
            dt: F::from(0.035).unwrap(),
        }
    }
}

/// A basic implementation
#[derive(Debug)]
pub struct StressMajorization<F: Field, const D: usize> {
    pub conf: Config<F>,
    pub shortest_path_matrix: HashMap<NodeIndex, HashMap<NodeIndex, F>>,
}

impl<F: Field + Measure + Sum, const D: usize> StressMajorization<F, D> {
    pub fn new(conf: Config<F>) -> Self {
        Self {
            conf,
            shortest_path_matrix: HashMap::new(),
        }
    }

    fn apply<N: Clone, E: Clone>(&mut self, graph: &mut StableGraph<(N, Point<F, D>), E>) {
        // if self.shortest_path_matrix.is_empty() {
        //     self.shortest_path_matrix = graph
        //         .node_indices()
        //         .map(|idx| {
        //             (idx, )
        //         })
        //         .collect();
        // }

        // let borrowed_graph: &StableGraph<(N, Point<F, D>), E> = graph;

        self.shortest_path_matrix.extend(
            graph
                .node_indices()
                .map(|idx| (idx, dijkstra(graph as &_, idx, None, |_| F::one()))),
        )
    }

    fn calc_stress<N: Clone, E: Clone>(
        &mut self,
        graph: &mut StableGraph<(N, Point<F, D>), E>,
    ) -> F {
        // graph
        //     .node_indices()
        //     .flat_map(|v| {
        //         graph.node_indices().skip(v.index() + 1).map(move |w| {
        //             let dist = nalgebra::distance(
        //                 &graph.node_weight(v).unwrap().1,
        //                 &graph.node_weight(w).unwrap().1,
        //             );

        //             if dist != F::zero() {
        //                 let dij = self.shortest_path_matrix[&v][&w];

        //                 let sp_diff = self.shortest_path_matrix[&v][&w] - dist;
        //                 dij.simd_sqrt().abs() * sp_diff * sp_diff
        //             } else {
        //                 F::zero()
        //             }
        //         })
        //     })
        //     .sum()
        F::default()
    }
}

// impl<const D: usize, N, E> Force<f32, D, N, E> for StressMajorization<D> {
//     fn apply(&mut self, graph: &mut ForceGraph<f32, D, N, E>) {
//         if self.shortest_path_matrix.is_empty() {
//             self.shortest_path_matrix = graph
//                 .node_indices()
//                 .map(|idx| (idx, petgraph::algo::dijkstra(graph, idx, None, |e| 1.0)))
//                 .collect();
//         }

//         let start_positions: HashMap<NodeIndex, Point<f32, D>, HashFn> = graph
//             .node_indices()
//             .map(|idx| (idx, graph.node_weight(idx).unwrap().1))
//             .collect();

//         for idx in start_positions.keys() {
//             let mut velocity: SVector<f32, D> = *self.velocities.get(idx).unwrap_or(&SVector::<
//                 f32,
//                 D,
//             >::zeros(
//             ));

//             let pos = start_positions.get(idx).unwrap();

//             let attraction = graph
//                 .neighbors_undirected(*idx)
//                 .filter(|neighbor_idx| neighbor_idx != idx)
//                 .map(|neighbor_idx| start_positions.get(&neighbor_idx).unwrap())
//                 .map(|neighbor_pos| {
//                     (neighbor_pos - pos).normalize()
//                         * (nalgebra::distance_squared(neighbor_pos, pos) / self.conf.scale)
//                 })
//                 .sum::<SVector<f32, D>>();
//             let repulsion = graph
//                 .node_indices()
//                 .filter(|other_idx| other_idx != idx)
//                 .map(|other_idx| start_positions.get(&other_idx).unwrap())
//                 .map(|other_pos| {
//                     (other_pos - pos).normalize()
//                         * -(self.conf.scale.simd_powi(2)
//                             / nalgebra::distance_squared(other_pos, pos))
//                 })
//                 .sum::<SVector<f32, D>>();

//             velocity.add_assign((attraction + repulsion) * self.conf.dt);
//             velocity.scale_mut(self.conf.cooloff_factor);

//             self.velocities.insert(*idx, velocity);

//             graph
//                 .node_weight_mut(*idx)
//                 .unwrap()
//                 .1
//                 .add_assign(velocity * self.conf.dt);
//         }
//     }
// }
