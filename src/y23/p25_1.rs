use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::repeat;
use std::slice::Iter;

// TODO: Make it generic not just for &str, just anything that is hashable, then export it to lib
// TODO: Add translation back using a Vec
#[derive(Default)]
struct Dictionary<'a> {
    lookup: RefCell<HashMap<&'a str, usize>>,
}

// A dictionary useful for translating ridiculous keys to usize which can be used for graphs
impl<'a> Dictionary<'a> {
    pub fn id(&self, name: &'a str) -> usize {
        let lookup = self.lookup.borrow();
        if let Some(id) = lookup.get(name) {
            *id
        } else {
            let new_id = lookup.len();
            drop(lookup);

            let mut lookup = self.lookup.borrow_mut();
            lookup.insert(name, new_id);
            new_id
        }
    }
}

#[derive(Default, Clone)]
struct FlexibleGraph {
    pub neighbors: Vec<Vec<usize>>,
    pub edge_indices: Vec<Vec<usize>>,

    pub edges: Vec<[usize; 2]>,
}

// TODO: Maybe weights, cost, will be needed in the future?
impl FlexibleGraph {
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

    pub fn add_undirected(&mut self, u: usize, v: usize) {
        let edge_id = self.edges.len();

        self.edges.push([u, v]);

        self.add_directed_with_edge_id(u, edge_id, v);
        self.add_directed_with_edge_id(v, edge_id, u);
    }
}

struct DfsSpecial<'a> {
    graph:  &'a FlexibleGraph,
    vis: Vec<bool>,
    ignore: [usize; 3],
}

impl<'a> DfsSpecial<'a> {
    pub fn new(graph: &'a FlexibleGraph) -> Self {
        Self {
            graph: &graph,
            vis: vec![false; graph.number_of_vertices()],
            ignore: [0, 0, 0],
        }
    }
    pub fn reset(&mut self, ignore: [usize; 3]) {
        self.ignore = ignore;
        self.vis.fill(false);
    }

    fn dfs(&mut self, u: usize, p: usize) -> i32 {
        let mut count = 1;
        self.vis[u] = true;
        for (&v, ei) in self.graph.neighbors[u].iter().zip(self.graph.edge_indices[u].iter()) {
            if v != p && !self.vis[v] && !self.ignore.contains(ei) {
                count += self.dfs(v, u);
            }
        }
        count
    }
    pub fn number_of_components(&mut self) -> Vec<i32> {
        let mut sizes = vec![];
        for u in 0..self.graph.number_of_vertices() {
            if !self.vis[u] {
                sizes.push(self.dfs(u, u));
            }
        }
        sizes
    }
}


pub fn run(content: &str) -> i32 {
    let dict = Dictionary::default();
    let mut graph = FlexibleGraph::default();
    for line in content.lines() {
        let mut it = line.split(": ");
        let u = dict.id(it.next().unwrap());

        it.next().unwrap().split(' ').for_each(|v| {
            graph.add_undirected(u, dict.id(v));
        })
    }
    // LOCK IT IN
    let graph = graph;

    let mut dfs = DfsSpecial::new(&graph);
    println!("N={} E={}", graph.number_of_vertices(), graph.edges.len());

    for i in 0..graph.edges.len() {
        for j in 0..i {
            for k in 0..j {
                dfs.reset([i, j, k]);
                let sizes = dfs.number_of_components();
                if sizes.len() == 2 {
                    return sizes[0] * sizes[1];
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::run;

    #[test]
    pub fn test_example() {
        let content = r##"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"##;

        assert_eq!(run(content), 54);
    }
}