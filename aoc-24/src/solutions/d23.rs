use std::collections::{HashMap, HashSet};

use utils::read_lines;

#[derive(Debug)]
struct Lan {
    graph: HashMap<String, HashSet<String>>,
}

impl FromIterator<String> for Lan {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut graph = HashMap::new();

        for pair in iter {
            (*graph.entry(pair[..2].to_string()).or_insert(HashSet::new()))
                .insert(pair[3..].to_string());
        }

        Self { graph }
    }
}

impl Lan {
    fn find_connections(&self, start: &String, n: usize) {
        for node in &self.graph {}
    }
}

pub fn solve_a() {
    let res = Lan::from_iter(read_lines("d23.txt", 24));

    for k in res.graph.keys() {
        res.find_connections(k, 3);
    }
}
