use std::collections::HashMap;

use crate::graph::Graph;

type DegreeMap = HashMap<u32, Vec<usize>>;
type Isomorphism = HashMap<usize, usize>;

pub fn isomorphic(g1: &Graph, g2: &Graph) -> bool {
    if g1.size() != g2.size() {
        return false;
    }

    let mut degrees1 = DegreeMap::new();
    let mut degrees2 = DegreeMap::new();
    for v in 0..g1.size() {
        degrees1.entry(g1.degree(v)).or_default().push(v);
        degrees2.entry(g2.degree(v)).or_default().push(v);
    }

    if degrees1
        .iter()
        .any(|(degree, vs1)| degrees2[degree].len() != vs1.len())
    {
        return false;
    }
    let mut isomorphism = Isomorphism::new();
    isomorphic_recursive(&degrees1, &degrees2, &mut isomorphism)
}

pub fn isomorphic_recursive(
    degrees1: &DegreeMap,
    degrees2: &DegreeMap,
    isomorphism: &mut Isomorphism,
) -> bool {
    false
}
