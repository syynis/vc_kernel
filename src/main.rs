use std::path::Path;

use graph::Graph;
use isomorphism::{enumerate_non_isomorphic, isomorphic};
use itertools::Itertools;
use solve::{branch, Solution};

pub mod graph;
pub mod isomorphism;
pub mod profile;
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

    let profile_searcher =
        profile::ProfileSearcher::new(Path::new(&String::from("profile4-2.txt")));
    let profile = profile_searcher.search();
    println!("Original {:?}", profile);
    let profile_searcher =
        profile::ProfileSearcher::new(Path::new(&String::from("profile4-2-r.txt")));
    let profile = profile_searcher.search();
    println!("Reduced  {:?}", profile);

    let iso3 = enumerate_non_isomorphic(4);
}
