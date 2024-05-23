use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

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

    pub fn has_edge(&self, from: &V, to: &V) -> bool {
        // O(1)
        if let Some(adjacencies) = self.get_adjacencies(from) {
            // O(1)
            adjacencies.contains(&to.to_owned())
        } else {
            false
        }
    }

    pub fn dfs(&self, node: V) -> HashSet<V> {
        let mut visited: HashSet<V> = HashSet::new();
        let mut stack = vec![&node];

        // O(|V| + |E|)
        while let Some(node) = stack.pop() {
            visited.insert(node.clone());

            if let Some(adjacencies) = self.get_adjacencies(&node) {
                for adj in adjacencies {
                    if !visited.contains(adj) {
                        stack.push(adj);
                    }
                }
            }
        }

        visited
    }

    pub fn compute_ccs(&self) -> Vec<Vec<V>> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        let op = self.opposite();

        for node in self.nodes.iter() {
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

            // println!("CC: {:?}", cc);

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

            println!("All CC: {:?}", cc);

            let new_cc = Rc::new(RefCell::new(HashSet::new()));

            let mut stack: Vec<&V> = vec![node];

            while let Some(node) = stack.pop() {
                println!("New CC: {:?}", new_cc.borrow());

                if cc.contains_key(&node) {
                    // merge the two connected components and go to the next node

                    let old_cc: &Rc<RefCell<HashSet<V>>> = cc.get(&node).unwrap();

                    println!(
                        "Merging {:?} with {:?} due to link to {:?}",
                        new_cc.borrow(),
                        old_cc.borrow(),
                        node
                    );

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
