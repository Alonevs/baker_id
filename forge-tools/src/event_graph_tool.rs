pub struct EventGraphTool {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

impl EventGraphTool {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: String) {
        self.nodes.push(name);
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.push((from, to));
    }
}
