use std::{collections::HashMap, fs::read_to_string, path::Path};

use itertools::Itertools;

#[derive(Debug)]
pub struct AdjMatrix {
    m: Vec<bool>,
    pub size: usize,
}

impl AdjMatrix {
    pub fn new(size: usize) -> Self {
        Self {
            m: vec![false; size * size],
            size,
        }
    }

    pub fn has_edge(&self, v: usize, u: usize) -> bool {
        assert!(v <= u);
        self.m[v * self.size + u]
    }

    pub fn set_edge(&mut self, v: usize, u: usize, val: bool) {
        assert!(v <= u);
        self.m[v * self.size + u] = val;
    }

    pub fn advance(&mut self) -> bool {
        let mut j = 0;
        for i in 0..self.size {
            j = i + 1;
            while j < self.size {
                if !self.has_edge(i, j) {
                    self.set_edge(i, j, true);
                    break;
                } else {
                    self.set_edge(i, j, false);
                }
                j += 1;
            }
            if j < self.size {
                break;
            }
        }
        j < self.size
    }

    pub fn next_bipartite(&mut self, split_idx: usize) -> bool {
        let mut j = split_idx;

        for i in 0..split_idx {
            j = split_idx;
            while j < self.size {
                if !self.has_edge(i, j) {
                    self.set_edge(i, j, true);
                    break;
                } else {
                    self.set_edge(i, j, false);
                }
                j += 1;
            }
            if j < self.size {
                break;
            }
        }
        j < self.size
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub neighbors: Vec<Vec<usize>>,
    degrees: Vec<u32>,
    pub valid: Vec<bool>,
    names: Vec<String>,
    id_name_map: HashMap<String, usize>,
    pub vertices: u32,
}

impl From<&AdjMatrix> for Graph {
    fn from(value: &AdjMatrix) -> Self {
        let mut g = Graph::empty();
        let size = value.size;
        (0..size).for_each(|i| g.add_vertex(format!("{}", i)));
        for i in 0..size {
            for j in (i + 1)..size {
                if value.has_edge(i, j) {
                    g.add_edge(i, j);
                }
            }
        }
        g
    }
}

impl From<String> for Graph {
    fn from(value: String) -> Self {
        let mut map: HashMap<String, usize> = HashMap::new();
        let mut graph = Self::empty();
        value.trim().split('\n').for_each(|e| {
            let x = e.split(' ').collect_vec();
            assert!(x.len() == 2);
            let v = *map.entry(x[0].to_owned()).or_insert_with(|| {
                graph.add_vertex(x[0].to_owned());
                graph.vertices as usize - 1
            });
            let u = *map.entry(x[1].to_owned()).or_insert_with(|| {
                graph.add_vertex(x[1].to_owned());
                graph.vertices as usize - 1
            });
            graph.add_edge(v, u);
        });
        graph.set_map(map);
        graph
    }
}

impl Graph {
    pub fn empty() -> Self {
        Self {
            neighbors: Vec::new(),
            degrees: Vec::new(),
            valid: Vec::new(),
            names: Vec::new(),
            id_name_map: HashMap::new(),
            vertices: 0,
        }
    }

    fn set_map(&mut self, map: HashMap<String, usize>) {
        self.id_name_map = map;
    }

    pub fn name_id(&self, name: String) -> usize {
        self.id_name_map[&name]
    }

    pub fn id_name(&self, id: usize) -> String {
        self.names[id].clone()
    }

    pub fn read(file: &Path) -> Self {
        let edgelist = read_to_string(file).unwrap();
        Self::from(edgelist)
    }

    pub fn add_vertex(&mut self, name: String) {
        self.neighbors.push(Vec::new());
        self.degrees.push(0);
        self.valid.push(true);
        self.names.push(name);
        self.vertices += 1;
    }

    pub fn invalidate_vertex(&mut self, v: usize) {
        if !self.valid[v] {
            println!("{} invalidated when invalid", v);
            return;
        }
        for n in &self.neighbors[v] {
            self.degrees[*n] -= 1;
        }
        self.vertices -= 1;
        self.valid[v] = false;
    }

    pub fn revalidate_vertex(&mut self, v: usize) {
        if self.valid[v] {
            println!("{} revalidated when valid", v);
            return;
        }
        self.valid[v] = true;
        self.vertices += 1;
        for n in &self.neighbors[v] {
            self.degrees[*n] += 1;
        }
    }

    pub fn add_edge(&mut self, v: usize, u: usize) {
        assert!(self.valid[v]);
        assert!(self.valid[u]);
        self.neighbors[v].push(u);
        self.neighbors[u].push(v);
        self.degrees[v] += 1;
        self.degrees[u] += 1;
    }

    pub fn degree(&self, v: usize) -> u32 {
        assert!(
            self.neighbors[v]
                .iter()
                .filter(|x| self.valid[**x])
                .collect_vec()
                .len()
                == self.degrees[v] as usize
        );
        self.degrees[v]
    }

    pub fn size(&self) -> usize {
        self.neighbors.len()
    }

    pub fn has_edge(&self, v: usize, u: usize) -> bool {
        self.neighbors[v].iter().any(|&x| x == u)
    }

    pub fn num_edges(&self) -> u32 {
        self.valid
            .iter()
            .enumerate()
            .map(|(id, &x)| if x { self.degree(id) } else { 0 })
            .sum::<u32>()
            / 2
    }
}
