use rayon::prelude::*;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::iter::repeat;
use std::ops::{Deref, DerefMut};
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use crate::utils::dict::Dictionary;
use crate::utils::graph::FlexibleGraph;


struct DfsSpecial<'a> {
    graph: &'a FlexibleGraph,
    low: Vec<i32>,
    disc: Vec<i32>,
    time: usize,
    ignore: [usize; 2],
    ans: Option<usize>,
}

impl<'a> DfsSpecial<'a> {
    pub fn new(graph: &'a FlexibleGraph, ignore: [usize; 2]) -> Self {
        let n = graph.number_of_vertices();
        Self {
            graph,
            ignore,

            ans: None,
            disc: vec![-1; n],
            low: vec![-1; n],
            time: 0,
        }
    }
    pub fn reset(&mut self, ignore: [usize; 2]) {
        self.ignore = ignore;
        self.ans = None;
        self.disc.fill(-1);
        self.low.fill(-1);
        self.time = 0;
    }

    fn dfs(&mut self, u: usize, p: usize) {
        self.time += 1;
        self.disc[u] = self.time as i32;
        self.low[u] = self.time as i32;

        for (&v, ei) in self.graph.neighbors[u].iter().zip(self.graph.edge_indices[u].iter()) {
            if v == p || self.ignore.contains(ei) { continue; }
            if self.disc[v] == -1 {
                let before_time = self.time;
                self.dfs(v, u);
                if self.low[v] > self.disc[u] {
                    // It's assuming that there is only one bridge
                    let subtree_size = self.time - before_time;
                    let other_size = self.graph.number_of_vertices() - subtree_size;
                    self.ans = Some(subtree_size * other_size);
                }
                self.low[u] = min(self.low[u], self.low[v]);
            } else { // back edge
                self.low[u] = min(self.low[u], self.disc[v]);
            }
        }
    }
    pub fn solve(&mut self) -> Option<usize> {
        for u in 0..self.graph.number_of_vertices() {
            if self.disc[u] == -1 {
                self.dfs(u, u);
            }
        }
        self.ans
    }
}


pub fn run(content: &str) -> usize {
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

    println!("N={} E={}", graph.number_of_vertices(), graph.edges.len());

    let m = graph.number_of_edges();
    let iter: Vec<(usize, usize)> = (0..m).flat_map(|i| (0..i).map(move |j| (i, j))).collect();

    let cnt = Arc::new(Mutex::new(0usize));
    let ans: Vec<_> = iter.par_iter().flat_map(|(i, j)| {
        let mut dfs = DfsSpecial::new(&graph, [*i, *j]);
        dfs.solve()
    }).collect();

    ans[0]
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use color_eyre::Result;
    use crate::y23::p25_1;
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

        // https://dreampuf.github.io/GraphvizOnline/#graph%20G%20%7B%0A%20%20%20%20jqt%20--%20rhn%3B%0A%20%20%20%20jqt%20--%20xhk%3B%0A%20%20%20%20jqt%20--%20nvd%20%5Bcolor%3Dred%5D%3B%0A%20%20%20%20%0A%20%20%20%20rsh%20--%20frs%3B%0A%20%20%20%20rsh%20--%20pzl%3B%0A%20%20%20%20rsh%20--%20lsr%3B%0A%20%20%20%20%0A%20%20%20%20xhk%20--%20hfx%3B%0A%20%20%20%20%0A%20%20%20%20cmg%20--%20qnr%3B%0A%20%20%20%20cmg%20--%20nvd%3B%0A%20%20%20%20cmg%20--%20lhk%3B%0A%20%20%20%20cmg%20--%20bvb%20%5Bcolor%3Dblue%5D%3B%0A%20%20%20%20%0A%20%20%20%20rhn%20--%20xhk%3B%0A%20%20%20%20rhn%20--%20bvb%3B%0A%20%20%20%20rhn%20--%20hfx%3B%0A%20%20%20%20%0A%20%20%20%20bvb%20--%20xhk%3B%0A%20%20%20%20bvb%20--%20hfx%3B%0A%20%20%20%20%0A%20%20%20%20pzl%20--%20lsr%3B%0A%20%20%20%20pzl%20--%20hfx%20%5Bcolor%3Dgreen%5D%3B%0A%20%20%20%20pzl%20--%20nvd%3B%0A%20%20%20%20%0A%20%20%20%20qnr%20--%20nvd%3B%0A%20%20%20%20%0A%20%20%20%20ntq%20--%20jqt%3B%0A%20%20%20%20ntq%20--%20hfx%3B%0A%20%20%20%20ntq%20--%20bvb%3B%0A%20%20%20%20ntq%20--%20xhk%3B%0A%20%20%20%20%0A%20%20%20%20nvd%20--%20lhk%3B%0A%20%20%20%20%0A%20%20%20%20lsr%20--%20lhk%3B%0A%20%20%20%20%0A%20%20%20%20rzs%20--%20qnr%3B%0A%20%20%20%20rzs%20--%20cmg%3B%0A%20%20%20%20rzs%20--%20lsr%3B%0A%20%20%20%20rzs%20--%20rsh%3B%0A%20%20%20%20%0A%20%20%20%20frs%20--%20qnr%3B%0A%20%20%20%20frs%20--%20lhk%3B%0A%20%20%20%20frs%20--%20lsr%3B%0A%7D%0A
        assert_eq!(run(content), 54);
    }

    #[ignore] // Too slow for automated tests
    #[test]
    pub fn test_input() -> Result<()> {
        let str = read_to_string("25.in")?;
        let ans = run(&str);
        assert_eq!(580800, ans);
        Ok(())
    }
}