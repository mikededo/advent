use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use utils::read_bytes_with;

struct Graph {
    start: (usize, usize),
    end: (usize, usize),
    nodes: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Graph {
    fn is_edge(curr: u8, next: u8) -> bool {
        let curr = match curr {
            b'S' => b'a',
            _ => curr,
        };

        let next = match next {
            b'E' => b'z',
            _ => next,
        };

        (next as i8 - curr as i8) <= 1
    }

    pub fn new(map: Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Self {
        let mut graph = Self {
            nodes: HashMap::new(),
            start,
            end,
        };

        map.iter().enumerate().for_each(|(x, r)| {
            r.iter().enumerate().for_each(|(y, c)| {
                let mut edges = Vec::new();

                // Check all four directions
                if x > 0 && Graph::is_edge(*c, map[x - 1][y]) {
                    edges.push((x - 1, y));
                }
                if x < map.len() - 1 && Graph::is_edge(*c, map[x + 1][y]) {
                    edges.push((x + 1, y));
                }
                if y > 0 && Graph::is_edge(*c, map[x][y - 1]) {
                    edges.push((x, y - 1));
                }
                if y < r.len() - 1 && Graph::is_edge(*c, map[x][y + 1]) {
                    edges.push((x, y + 1));
                }

                graph.nodes.insert((x, y), edges);
            });
        });

        graph
    }

    pub fn find_shortest(self) -> usize {
        let mut queue = BinaryHeap::new();
        let mut track = HashMap::from([(self.start, 0usize)]);

        queue.push(Reverse((0, self.start)));

        while let Some(Reverse((dist, node))) = queue.pop() {
            if let Some(&prev_dist) = track.get(&node) {
                if dist > prev_dist {
                    continue;
                }
            }

            if node == self.end {
                return dist;
            }

            for next in self.nodes.get(&node).unwrap_or(&vec![]) {
                let cost = dist + 1;
                if track.get(next).map_or(true, |&c| cost < c) {
                    queue.push(Reverse((cost, *next)));
                    track.insert(*next, cost);
                }
            }
        }

        usize::MAX
    }
}

pub fn solve_a() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = read_bytes_with("d12.txt", 22, |c, point| match c {
        b'S' => start = point,
        b'E' => end = point,
        _ => (),
    });
    println!("{}", Graph::new(map, start, end).find_shortest());
}
