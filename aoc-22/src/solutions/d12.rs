use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use utils::read_bytes_with;

struct Graph {
    nodes: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

fn is_edge_a(curr: u8, next: u8) -> bool {
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

fn is_edge_b(curr: u8, next: u8) -> bool {
    let curr = match curr {
        b'E' => b'z',
        _ => curr,
    };

    let next = match next {
        b'S' => b'a',
        _ => next,
    };

    (curr as i8 - next as i8) <= 1
}

impl Graph {
    pub fn new(map: &[Vec<u8>], is_edge: fn(u8, u8) -> bool) -> Self {
        let mut graph = Self {
            nodes: HashMap::new(),
        };

        map.iter().enumerate().for_each(|(x, r)| {
            r.iter().enumerate().for_each(|(y, c)| {
                let mut edges = Vec::new();

                // Check all four directions
                if x > 0 && is_edge(*c, map[x - 1][y]) {
                    edges.push((x - 1, y));
                }
                if x < map.len() - 1 && is_edge(*c, map[x + 1][y]) {
                    edges.push((x + 1, y));
                }
                if y > 0 && is_edge(*c, map[x][y - 1]) {
                    edges.push((x, y - 1));
                }
                if y < r.len() - 1 && is_edge(*c, map[x][y + 1]) {
                    edges.push((x, y + 1));
                }

                graph.nodes.insert((x, y), edges);
            });
        });

        graph
    }

    pub fn find_shortest<F>(self, start: (usize, usize), is_end: F) -> usize
    where
        F: Fn((usize, usize)) -> bool,
    {
        let mut queue = BinaryHeap::new();
        let mut track = HashMap::new();

        track.insert(start, 0);
        queue.push(Reverse((0, start)));

        while let Some(Reverse((dist, node))) = queue.pop() {
            if let Some(&prev_dist) = track.get(&node) {
                if dist > prev_dist {
                    continue;
                }
            }

            if is_end(node) {
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
    println!(
        "{}",
        Graph::new(&map, is_edge_a).find_shortest(start, |node| node == end)
    );
}

pub fn solve_b() {
    let mut start = (0, 0);
    let map = read_bytes_with("d12.txt", 22, |c, point| {
        if c == b'E' {
            start = point
        }
    });

    println!(
        "{}",
        Graph::new(&map, is_edge_b).find_shortest(start, |(x, y)| map[x][y] == b'a')
    );
}
