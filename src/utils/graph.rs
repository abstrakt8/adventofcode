use core::default::Default;

#[derive(Default, Clone)]
pub struct FlexibleGraph {
    pub neighbors: Vec<Vec<usize>>,
    pub edge_indices: Vec<Vec<usize>>,

    pub edges: Vec<[usize; 2]>,
}


// TODO: Maybe weights, cost, will be needed in the future?
impl FlexibleGraph {
    pub fn new() -> Self { FlexibleGraph { ..Default::default() } }
    fn ensure_size(&mut self, idx: usize) {
        // condition required otherwise resize_with will truncate the vector
        if self.neighbors.len() <= idx {
            self.neighbors.resize_with(idx + 1, Default::default);
            self.edge_indices.resize_with(idx + 1, Default::default);
        }
    }

    fn add_directed_with_edge_id(&mut self, u: usize, edge_id: usize, v: usize) {
        self.ensure_size(u);
        self.neighbors[u].push(v);
        self.edge_indices[u].push(edge_id);
    }

    pub fn number_of_vertices(&self) -> usize {
        self.neighbors.len()
    }

    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn add_undirected(&mut self, u: usize, v: usize) {
        let edge_id = self.edges.len();

        self.edges.push([u, v]);

        self.add_directed_with_edge_id(u, edge_id, v);
        self.add_directed_with_edge_id(v, edge_id, u);
    }

    pub fn add_directed(&mut self, u: usize, v: usize) {
        let edge_id = self.edges.len();

        self.edges.push([u, v]);

        self.add_directed_with_edge_id(u, edge_id, v);
    }
}
