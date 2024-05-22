use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Debug)]
pub struct AdjacencyGraph {
    nodes: HashMap<String, String>,
    adjacencies: HashMap<String, HashSet<String>>,
}

#[allow(dead_code)]
impl AdjacencyGraph {
    pub fn new() -> Self {
        AdjacencyGraph {
            nodes: HashMap::new(),
            adjacencies: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, key: String, value: String) {
        self.nodes.insert(key, value);
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.adjacencies
            .entry(from)
            .or_insert(HashSet::new())
            .insert(to);
    }

    pub fn get_adjacencies(&self, node: &str) -> Option<&HashSet<String>> {
        self.adjacencies.get(node)
    }

    pub fn adjacencies(&self) -> &HashMap<String, HashSet<String>> {
        &self.adjacencies
    }

    pub fn opposite(&self) -> AdjacencyGraph {
        let mut opposite = AdjacencyGraph::new();

        for (from, adjacencies) in self.adjacencies.iter() {
            for to in adjacencies {
                opposite.add_edge(to.clone(), from.clone());
            }
        }

        opposite
    }

    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        if let Some(adjacencies) = self.get_adjacencies(from) {
            adjacencies.contains(&to.to_string())
        } else {
            false
        }
    }

    // pub fn dfs(&self, node: String) -> HashSet<String> {
    //     let mut visited = HashSet::new();
    //     let mut stack = vec![node];

    //     while let Some(node) = stack.pop() {
    //         if visited.contains(&node) {
    //             continue;
    //         }

    //         visited.insert(node.clone());

    //         if let Some(adjacencies) = self.get_adjacencies(&node) {
    //             for adj in adjacencies {
    //                 stack.push(adj.clone());
    //             }
    //         }
    //     }

    //     visited
    // }

    pub fn compute_ccs(&self) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        let op = self.opposite();

        for node in self.nodes.keys() {
            if visited.contains(node) {
                continue;
            }

            let mut cc = HashSet::new();
            let mut stack = vec![node.to_string()];

            while let Some(node) = stack.pop() {
                if cc.contains(&node) {
                    continue;
                }

                cc.insert(node.clone());

                if let Some(adjacencies) = self.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj.clone());
                    }
                }

                if let Some(adjacencies) = op.get_adjacencies(&node) {
                    for adj in adjacencies {
                        stack.push(adj.clone());
                    }
                }
            }

            visited.extend(cc.iter().map(|x| x.to_owned()));
            result.push(cc.iter().map(|x| x.to_owned()).collect());
        }

        result
    }

    pub fn compute_ccs_2(&self) -> Vec<Vec<String>> {
        let mut cc = HashMap::<String, Rc<RefCell<HashSet<String>>>>::new();

        for node in self.nodes.keys() {
            if cc.contains_key(node) {
                continue;
            }

            println!("All CC: {:?}", cc);

            let new_cc = Rc::new(RefCell::new(HashSet::new()));

            let mut stack = vec![node.to_string()];

            while let Some(node) = stack.pop() {
                println!("New CC: {:?}", new_cc.borrow());

                if cc.contains_key(&node) {
                    // merge the two connected components and go to the next node

                    let old_cc = cc.get(&node).unwrap();

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
                        stack.push(adj.clone());
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

        for node in self.nodes.keys() {
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
