struct AdjacencyGraph {
    nodes: Vec<&str>,
    adjacencies: HashMap<&str, Vec<&str>>,
}

impl AdjacencyGraph {
    fn new() -> Self {
        AdjacencyGraph {
            nodes: Vec::new(),
            adjacencies: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &str) {
        self.nodes.push(node);
        self.adjacencies.insert(node, Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacencies.get_mut(from).unwrap().push(to);
    }

    fn neighbors(&self, node: &str) -> Option<&Vec<&str>> {
        self.adjacencies.get(node)
    }
}
