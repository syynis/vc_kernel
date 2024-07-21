use itertools::Itertools;

use crate::{
    graph::{AdjMatrix, Graph},
    isomorphism::enumerate_non_isomorphic,
    solve::{branch, select_vertex, Solution},
};

#[derive(Debug)]
pub struct Profile(Vec<u32>);

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        };

        let offset = other.0[0] - self.0[0];
        for i in 1..self.0.len() {
            if offset + self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.len().trailing_zeros();
        self.0.chunks(size as usize).for_each(|c| {
            let _ = writeln!(f, "{:?}", c.to_owned());
        });
        Ok(())
    }
}

pub struct ProfileSearcher {
    g: Graph,
    // This maps border vertices to their ids in the graph
    border: Vec<usize>,
}

impl ProfileSearcher {
    pub fn new(g: Graph, border: Vec<usize>) -> Self {
        Self { g, border }
    }

    pub fn load(file: &std::path::Path) -> Self {
        let file_content = std::fs::read_to_string(file).unwrap();
        let (header, edge_list) = file_content.split_once('\n').unwrap();

        let g = Graph::from(edge_list.to_owned());
        let border = header
            .split_whitespace()
            .map(|name| g.name_id(name.to_owned()))
            .collect_vec();
        ProfileSearcher { g, border }
    }
    pub fn search(&self) -> Profile {
        let profile_size = self.border.len();
        let mut res = vec![0; 1 << profile_size];
        let mut border = vec![false; profile_size];
        let mut index = 0;

        loop {
            let mut gprime = self.g.clone();
            let mut s = Solution::new(&gprime);
            let mut best = Solution::max(&gprime);
            border.iter().enumerate().for_each(|(idx, border_state)| {
                if *border_state {
                    gprime.invalidate_vertex(self.border[idx])
                } else {
                    for n in gprime.neighbors[self.border[idx]].clone() {
                        if gprime.valid[n] {
                            select_vertex(&mut gprime, n, &mut s);
                        }
                    }
                    gprime.invalidate_vertex(self.border[idx]);
                }
            });
            branch(&mut gprime, &mut s, &mut best);
            res[index] = best.size();
            index += 1;
            if !advance_border_solution(&mut border) {
                break;
            }
        }
        Profile(res)
    }
    pub fn search_equivalencies(&self, profile: &Profile) -> Vec<Graph> {
        let mut created = Vec::new();

        for iso_graph in enumerate_non_isomorphic(self.g.size() - self.border.len()) {
            let mut to_solve = iso_graph.clone();
            let mut sol = Solution::new(&iso_graph);
            let mut best = Solution::max(&iso_graph);
            branch(&mut to_solve, &mut sol, &mut best);
            if best.size() <= *profile.0.last().unwrap() {
                let mut bipartite = AdjMatrix::new(self.g.size());
                loop {
                    let mut equiv_candidate = iso_graph.clone();
                    let split = self.g.size() - self.border.len();
                    for i in 0..self.border.len() {
                        equiv_candidate.add_vertex(self.g.id_name(self.border[i]));
                    }
                    for i in 0..split {
                        for j in split..split + self.border.len() {
                            if bipartite.has_edge(i, j) {
                                equiv_candidate.add_edge(i, j);
                            }
                        }
                    }

                    let searcher =
                        ProfileSearcher::new(equiv_candidate.clone(), self.border.clone());
                    let other_profile = searcher.search();

                    if profile.eq(&other_profile) {
                        created.push(equiv_candidate.clone());
                    }

                    if !bipartite.next_bipartite(split) {
                        break;
                    }
                }
            }
        }
        created
    }
}

type BorderSolution = Vec<bool>;

fn advance_border_solution(border: &mut BorderSolution) -> bool {
    for i in border.iter_mut() {
        if *i {
            *i = false;
        } else {
            *i = true;
            return true;
        }
    }
    false
}
