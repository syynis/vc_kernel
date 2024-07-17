use std::path::Path;

use graph::Graph;
use solve::{branch, Solution};

pub mod graph;
pub mod isomorphism;
pub mod solve;

fn main() {
    let mut g = Graph::read(Path::new(&String::from("test.txt")));
    let mut curr = Solution::new(&g);
    let mut best = Solution::max(&g);
    branch(&mut g, &mut curr, &mut best);
    println!(
        "{:?}",
        best.flag
            .iter()
            .enumerate()
            .filter(|&(_, x)| *x)
            .map(|(v, _)| v)
    );
}
