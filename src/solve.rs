use crate::graph::Graph;

#[derive(Clone)]
pub struct Solution {
    pub flag: Vec<bool>,
    size: u32,
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.size.eq(&other.size)
    }
}
impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl Solution {
    pub fn new(g: &Graph) -> Self {
        Self {
            flag: vec![false; g.vertices as usize],
            size: 0,
        }
    }

    pub fn max(g: &Graph) -> Self {
        Self {
            flag: vec![true; g.vertices as usize],
            size: g.vertices,
        }
    }

    pub fn include(&mut self, v: usize) {
        self.flag[v] = true;
        self.size += 1;
    }

    pub fn exclude(&mut self, v: usize) {
        self.flag[v] = false;
        self.size -= 1;
    }

    pub fn in_sol(&self, v: usize) -> bool {
        self.flag[v]
    }
}

fn select_vertex(g: &mut Graph, v: usize, sol: &mut Solution) {
    sol.include(v);
    g.invalidate_vertex(v);
}

fn unselect_vertex(g: &mut Graph, v: usize, sol: &mut Solution) {
    sol.exclude(v);
    g.revalidate_vertex(v);
}

fn deg1_reduce(g: &mut Graph, sol: &mut Solution, removed: &mut Vec<usize>) -> bool {
    let mut reduced = false;
    for v in 0..g.size() {
        if !g.valid[v] {
            continue;
        };
        if g.degree(v) == 0 {
            g.invalidate_vertex(v);
            removed.push(v);
        } else if g.degree(v) == 1 {
            reduced = true;
            let u = g.neighbors[v]
                .iter()
                .find(|x| g.valid[**x])
                .cloned()
                .unwrap();
            select_vertex(g, u, sol);
            g.invalidate_vertex(v);
            removed.push(u);
            removed.push(v);
        }
    }
    reduced
}

fn max_degree(g: &Graph) -> usize {
    let mut max = 0usize;
    for v in 0..g.size() {
        if !g.valid[v] {
            continue;
        };
        if g.degree(v) as usize > max {
            max = v;
        }
    }
    max
}

pub fn branch(g: &mut Graph, current: &mut Solution, best: &mut Solution) {
    let mut removed = Vec::new();
    let old = current.clone();

    loop {
        if !deg1_reduce(g, current, &mut removed) {
            break;
        }
    }
    if g.vertices == 0 {
        if current < best {
            *best = current.clone();
            for r in removed {
                g.revalidate_vertex(r);
                if old.in_sol(r) {
                    current.exclude(r);
                }
            }
        }
        return;
    }

    let max_deg = max_degree(g);

    select_vertex(g, max_deg, current);
    branch(g, current, best);
    unselect_vertex(g, max_deg, current);

    let neighbors = g.neighbors[max_deg]
        .iter()
        .filter(|x| g.valid[**x])
        .copied()
        .collect::<Vec<usize>>();
    for n in &neighbors {
        select_vertex(g, *n, current);
    }
    g.invalidate_vertex(max_deg);

    branch(g, current, best);

    g.revalidate_vertex(max_deg);
    for n in neighbors {
        unselect_vertex(g, n, current);
    }

    for r in removed {
        g.revalidate_vertex(r);
        if old.in_sol(r) {
            current.exclude(r);
        }
    }
}
