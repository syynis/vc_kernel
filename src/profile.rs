use itertools::Itertools;

use crate::{
    graph::Graph,
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
            writeln!(f, "{:?}", c.to_owned());
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
    pub fn new(file: &std::path::Path) -> Self {
        let file_content = std::fs::read_to_string(file).unwrap();
        let (header, edge_list) = file_content.split_once('\n').unwrap();

        let g = Graph::from(edge_list.to_owned());
        let border = header
            .split_whitespace()
            .map(|name| g.get_name(name.to_owned()))
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
