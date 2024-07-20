use std::{
    collections::{hash_map::Iter, HashMap},
    iter::Peekable,
};

use itertools::Itertools;

use crate::graph::{AdjMatrix, Graph};

type DegreeMap = HashMap<u32, Vec<usize>>;
type Isomorphism = HashMap<usize, usize>;

pub fn isomorphic(g1: &Graph, g2: &Graph) -> bool {
    if g1.size() != g2.size() {
        println!("Not isomorphic {} =/= {}", g1.size(), g2.size());
        return false;
    }

    let mut degrees1 = DegreeMap::new();
    let mut degrees2 = DegreeMap::new();
    for v in 0..g1.size() {
        degrees1.entry(g1.degree(v)).or_default().push(v);
        degrees2.entry(g2.degree(v)).or_default().push(v);
    }

    if degrees1.iter().any(|(degree, vs1)| {
        if let Some(degree_verts) = degrees2.get(degree) {
            degree_verts.len() != vs1.len()
        } else {
            true
        }
    }) {
        println!("Not isomorphic, degrees don't match");
        return false;
    }

    let mut isomorphism = Isomorphism::new();
    isomorphic_recursive(
        g1,
        g2,
        &degrees1,
        &degrees2,
        &mut degrees1.iter().peekable(),
        &mut isomorphism,
    )
}

fn isomorphic_recursive(
    g1: &Graph,
    g2: &Graph,
    degrees1: &DegreeMap,
    degrees2: &DegreeMap,
    current: &mut Peekable<Iter<u32, Vec<usize>>>,
    isomorphism: &mut Isomorphism,
) -> bool {
    if current.peek().is_none() {
        for (_, vertices) in degrees1.iter() {
            for v in vertices.iter() {
                for n in &g1.neighbors[*v] {
                    if !g2.has_edge(*isomorphism.get(v).unwrap(), *isomorphism.get(n).unwrap()) {
                        return false;
                    }
                }
            }
        }
        true
    } else {
        let vs1 = current.peek().unwrap().1;
        let vs2 = degrees2.get(current.peek().unwrap().0).unwrap().clone();

        for _ in vs2.iter().permutations(vs2.len()) {
            for (i, j) in vs1.iter().zip(vs2.iter()) {
                isomorphism.insert(*i, *j);
            }
            current.next();
            if isomorphic_recursive(g1, g2, degrees1, degrees2, current, isomorphism) {
                return true;
            }
        }

        false
    }
}

pub fn enumerate_non_isomorphic(num_v: usize) -> Vec<Graph> {
    let mut m = AdjMatrix::new(num_v);
    let mut created = Vec::new();

    loop {
        let g = Graph::from(&m);
        if created.iter().all(|c| !isomorphic(&g, c)) {
            created.push(g);
        }

        if !m.advance() {
            break;
        }
    }

    for g in &created {
        println!("{:?}", g);
    }
    created
}
