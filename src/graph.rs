use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

use indicatif::{ProgressIterator, ProgressStyle};

#[derive(Debug)]
pub struct AdjacencyGraph<V>
where
    V: Hash + Eq + Clone,
{
    nodes: HashSet<V>,
    adjacencies: HashMap<V, HashSet<V>>,
}

#[allow(dead_code)]
impl<V> AdjacencyGraph<V>
where
    V: Hash + Eq + Clone + Debug,
{
    pub fn new() -> Self {
        AdjacencyGraph {
            nodes: HashSet::new(),
            adjacencies: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: V) {
        // O(1)
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: V, to: V) {
        // O(1)
        self.add_node(from.clone());
        self.add_node(to.clone());

        // O(1)
        self.adjacencies
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn get_adjacencies(&self, node: &V) -> Option<&HashSet<V>> {
        self.adjacencies.get(node)
    }

    pub fn adjacencies(&self) -> &HashMap<V, HashSet<V>> {
        &self.adjacencies
    }

    pub fn edges(&self) -> impl Iterator<Item = (&V, &V)> {
        self.adjacencies
            .iter()
            .flat_map(|(from, tos)| tos.iter().map(move |to| (from, to)))
    }

    pub fn opposite(&self) -> AdjacencyGraph<&V> {
        let mut opposite = AdjacencyGraph::new();

        // O(|E|)
        for (from, to) in self.edges() {
            opposite.add_edge(to, from);
        }

        opposite
    }

    pub fn undirected(&self) -> AdjacencyGraph<&V> {
        let mut undirected = AdjacencyGraph::new();

        // O(|E|)
        for (from, to) in self.edges() {
            undirected.add_edge(from, to);
            undirected.add_edge(to, from);
        }

        undirected
    }

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        // O(1)
        if let Some(adjacencies) = self.get_adjacencies(from) {
            // O(1)
            adjacencies.contains(&to.to_owned())
        } else {
            false
        }
    }

    pub fn dfs<'a>(&'a self, node: &'a V) -> impl Iterator<Item = V> + 'a {
        let mut visited = HashSet::new();
        let mut stack = VecDeque::from([node]);

        std::iter::from_fn(move || {
            while let Some(node) = stack.pop_back() {
                if !visited.insert(node.clone()) {
                    continue;
                }

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    stack.extend(adjacencies);
                }

                return Some(node.clone());
            }
            None
        })
    }

    pub fn shortest_path_matrix(&self) -> HashMap<&V, HashMap<&V, usize>> {
        let mut result = HashMap::new();

        for node in self.nodes.iter() {
            let mut distances = HashMap::new();
            let mut visited = HashSet::new();
            let mut queue = VecDeque::from([node]);

            distances.insert(node, 0);

            while let Some(node) = queue.pop_front() {
                if visited.contains(node) {
                    continue;
                }

                visited.insert(node.clone());

                let distance = *distances.get(node).unwrap();

                if let Some(adjacencies) = self.get_adjacencies(node) {
                    for adj in adjacencies {
                        if !distances.contains_key(adj) {
                            distances.insert(adj, distance + 1);
                            queue.push_back(adj);
                        }
                    }
                }
            }

            result.insert(node, distances);
        }

        result
    }

    pub fn compute_ccs(&self) -> Vec<Vec<V>> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        let op = self.opposite();

        for node in self
            .nodes
            .iter()
            .progress()
            .with_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{prefix} {spinner} [{elapsed_precise}] [{wide_bar}] {pos}/{len}")
                    .unwrap(),
            )
            .with_prefix("computing connected components")
        {
            if visited.contains(node) {
                continue;
            }

            let mut cc: HashSet<V> = HashSet::new();
            let mut stack: Vec<&V> = vec![node];

            while let Some(node) = stack.pop() {
                if cc.contains(node) {
                    continue;
                }

                cc.insert(node.clone());

                if let Some(adjacencies) = self.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
                }

                if let Some(adjacencies) = op.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
                }
            }

            visited.extend(cc.iter().map(|x| x.to_owned()));
            result.push(cc.iter().map(|x| x.to_owned()).collect());
        }

        result
    }

    pub fn compute_ccs_2(&self) -> Vec<Vec<V>> {
        let mut cc: HashMap<V, Rc<RefCell<HashSet<V>>>> = HashMap::new();

        for node in self.nodes.iter() {
            if cc.contains_key(&node) {
                continue;
            }

            // println!("All CC: {:?}", cc);

            let new_cc = Rc::new(RefCell::new(HashSet::new()));

            let mut stack: Vec<&V> = vec![node];

            while let Some(node) = stack.pop() {
                // println!("New CC: {:?}", new_cc.borrow());

                if cc.contains_key(&node) {
                    // merge the two connected components and go to the next node

                    let old_cc: &Rc<RefCell<HashSet<V>>> = cc.get(&node).unwrap();

                    // println!(
                    //     "Merging {:?} with {:?} due to link to {:?}",
                    //     new_cc.borrow(),
                    //     old_cc.borrow(),
                    //     node
                    // );

                    new_cc
                        .borrow_mut()
                        .extend(old_cc.borrow().iter().map(|x| x.to_owned()));

                    break;
                }

                if new_cc.borrow().contains(&node) {
                    continue;
                }

                new_cc.borrow_mut().insert(node.clone());

                if let Some(adjacencies) = self.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj);
                    }
                }
            }

            for n in new_cc.borrow().iter() {
                cc.insert(n.to_owned(), new_cc.clone());
            }
        }

        // extract the unique connected components by pointers
        let mut result = Vec::new();
        let mut seen = HashSet::new();

        for node in self.nodes.iter() {
            if seen.contains(node) {
                continue;
            }

            let cc = cc.get(node).unwrap();
            seen.extend(cc.borrow().iter().map(|x| x.to_owned()));

            result.push(cc.borrow().iter().map(|x| x.to_owned()).collect());
        }

        result
    }
}
