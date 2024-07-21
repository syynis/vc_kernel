use std::path::Path;

use graph::Graph;
use isomorphism::{enumerate_non_isomorphic, enumerate_non_isomorphic_with_edges, isomorphic};
use itertools::Itertools;
use solve::{branch, Solution};

pub mod graph;
pub mod isomorphism;
pub mod profile;
pub mod solve;

fn main() {
    let profile_searcher =
        profile::ProfileSearcher::load(Path::new(&String::from("profile4-2.txt")));
    let profile = profile_searcher.search();
    println!("Original {:?}", profile);

    println!("Non isomorphic");
    enumerate_non_isomorphic(5)
        .iter()
        .for_each(|g| println!("{:?}", g));

    println!("Non reduceable non isomorphic");
    non_reduceable_ismorphisms(5)
        .iter()
        .for_each(|g| println!("{:?}", g));
}

fn non_reduceable_ismorphisms(num_v: usize) -> Vec<Graph> {
    enumerate_non_isomorphic(num_v)
        .iter()
        .filter(|g| {
            let dominate = g.neighbors.iter().any(|adj| adj.len() == num_v - 1);
            let funnel = (0..num_v).any(|i| {
                let mut g_e = (*g).clone();
                g_e.invalidate_vertex(i);
                g_e.num_edges() as usize == (num_v - 1) * (num_v - 2) / 2
            });
            !(dominate || funnel)
        })
        .cloned()
        .collect_vec()
}
