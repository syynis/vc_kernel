use std::path::Path;

use graph::Graph;
use isomorphism::isomorphic;
use itertools::Itertools;
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
            .collect_vec()
    );

    let g1 = Graph::read(Path::new(&String::from("iso1.txt")));
    let g2 = Graph::read(Path::new(&String::from("iso2.txt")));

    println!("isomorphic {}", isomorphic(&g1, &g2));
}
