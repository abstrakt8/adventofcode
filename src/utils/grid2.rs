use color_eyre::eyre::OptionExt;
use color_eyre::Result;
use itertools::Itertools;
use std::iter;
// Graph can be anything like Grid, Nodes+Edges, ...
// Algorithms can be run on the graph, but they should just share the trait


// Abstracting this type would be a next challenge
type CellRowCol = (u32, u32);
trait GridGraph {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn contains(&self, i: i32, j: i32) -> bool {
        (0..self.rows() as i32).contains(&i) &&
            (0..self.cols() as i32).contains(&j)
    }
}

struct CharGrid<'a> {
    rows: usize,
    cols: usize,
    grid: &'a [u8],
}

const NEW_LINE: u8 = b'\n';
impl GridGraph for CharGrid<'_> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }
}

// TODO: Type compile type specific neighbouring system (4, 8)
impl<'a> CharGrid<'a> {
    pub fn from_str(content: &'a str) -> Result<Self> {
        let bytes = content.as_bytes();

        // TODO: Make it resilient, maybe just treat it as a 1xN grid
        let cols = memchr::memchr(NEW_LINE, bytes).ok_or_eyre("No newline '\\n' found")?;

        let rows = bytes.len() / (cols + 1);

        // TODO: What happens if no new line at the end?
        // TODO: What if not divisible?

        Ok(Self {
            grid: bytes,
            rows,
            cols,
        })
    }
}


/// You can implement a custom neighbour by implementing GridNeighbours (e.g. lets say Knight jumping pattern)
/// These are mostly helpers when the order of visiting doesn't matter.
trait GridNeighbours {
    fn grid_neighbours<G: GridGraph>(graph: &G, pos: CellRowCol) -> impl Iterator<Item=CellRowCol>;
}

///
/// 010
/// 0x1
/// 010
///
///
struct GridNeighbours4;
const DIR_4: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];


///
/// 111
/// 0x1
/// 111
struct GridNeighbours8;

// impl GridNeighbours for GridNeighbours4 {
//     fn grid_neighbours<G: GridGraph>(graph: &G, pos: CellRowCol) -> impl Iterator<Item=CellRowCol> {
//         DIR_4.iter().filter_map(|(dr, dc)| {
//             let nrow = pos.0 as i32 + dr;
//             let ncol = pos.1 as i32 + dc;
//             if graph.contains(nrow, ncol) {
//                 Some((nrow, ncol))
//             } else {
//                 None
//             }
//         })
//     }
// }


impl Graph<CellRowCol> for CharGrid<'_> {
    fn neighbours(&self, u: CellRowCol) -> impl Iterator<Item=CellRowCol> {
        iter::empty()
    }
}
// Maybe rename to something else? e.g. edges() is something that not all will support I guess
trait Graph<U> {
    fn neighbours(&self, u: U) -> impl Iterator<Item=U>;
}

// trait IsAcyclic<T>: Graph<T>
// where
//     T: Hash,
// {
//     fn is_acyclic(&self) -> bool {
//
//     }
// }
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn from_str() {
        let grid = r##"..
X.
NN
"##;
        let char_grid = CharGrid::from_str(grid).expect("To be parsed correctly");
        // This is under the assumption there is a new line, which is always true otherwise the .expect() will fail for a 1xN grid
        assert_eq!(char_grid.cols, 2);
        assert_eq!(char_grid.rows, 3);
    }
}

//
// type HyperNodeType = usize;
//
// impl Graph<HyperNodeType> for CharGrid {
//     fn neighbours(&self, u: HyperNodeType) {
//         todo!()
//     }
// }
//
//
// pub fn is_acyclic<G: Graph<U> + ?Sized, U: Hash>(graph: &G) -> bool {}
//
// pub fn run(content: &str) -> u32 {
//     0
// }

