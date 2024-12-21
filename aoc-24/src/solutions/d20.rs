use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use utils::read_lines;

fn adjacent((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [
        (x.wrapping_sub(1), y),
        (x + (1), y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

fn n_adjacent(point: (usize, usize), n: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut result = HashSet::new();
    let mut queue = VecDeque::new();

    // Start from the given point, with step count = 0
    queue.push_back((point, 0));

    while let Some((current, steps)) = queue.pop_front() {
        if steps == n {
            result.insert(current); // Add to result only at exactly n steps
            continue;
        }

        for next in adjacent(current) {
            queue.push_back((next, steps + 1));
        }
    }

    result.into_iter()
}

struct Map {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromIterator<String> for Map {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut map = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for line in iter {
            let mut row = vec![' '; line.len()];
            line.chars().enumerate().for_each(|(i, c)| {
                if c == 'S' {
                    start = (map.len(), i);
                } else if c == 'E' {
                    end = (map.len(), i);
                }

                row[i] = c;
            });

            map.push(row);
        }

        Self { map, start, end }
    }
}

impl Map {
    fn find_cheats_at(&self, cheats: usize, threshold: usize) {
        let min_path = self.find_shortest();
        let min_dist = min_path.get(&self.end).unwrap();
        let mut rest = BTreeMap::new(); // Simply using a btree to view the groups

        for (point, steps) in &min_path {
            for adj in n_adjacent(*point, cheats) {
                if let Some(dist) = min_path.get(&adj) {
                    // This 2 drove me mad! We need to also add the cheat!
                    let final_dist = min_dist - dist + steps + cheats;
                    if final_dist < *min_dist && *min_dist - final_dist >= threshold {
                        *rest.entry(final_dist).or_insert(0) += 1;
                    }
                }
            }
        }

        println!("{:?}", rest.values().sum::<usize>());
    }

    fn find_cheats_within(&self, limit: usize, threshold: isize) {
        let min_path = self.find_shortest();
        let mut rest = BTreeMap::new();

        for (point, steps) in min_path.iter() {
            for mpoint in self.manhattan_neighbors(*point, limit) {
                if let Some(msteps) = min_path.get(&mpoint) {
                    let dist = self.manhattan_distance(*point, mpoint) as isize;
                    let saving = *msteps as isize - *steps as isize - dist;

                    if saving >= threshold {
                        *rest.entry(saving).or_insert(0) += 1;
                    }
                }
            }
        }

        println!("{}", rest.iter().map(|(_, &count)| count).sum::<i32>());
    }

    // Thoday was really hard, thanks reddit :)
    fn find_shortest(&self) -> HashMap<(usize, usize), usize> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back((self.start, 0));
        visited.insert(self.start, 0);

        while let Some((curr, steps)) = queue.pop_front() {
            if curr == self.end {
                return visited;
            }

            for next in adjacent(curr) {
                if !self.is_safe(next) || visited.contains_key(&next) {
                    continue;
                }

                visited.insert(next, steps + 1);
                queue.push_back(((next), steps + 1));
            }
        }

        unreachable!("A path should've been found");
    }

    fn manhattan_distance(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let x = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
        let y = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };

        x + y
    }

    fn manhattan_neighbors(&self, point: (usize, usize), limit: usize) -> HashSet<(usize, usize)> {
        let mut neighbors = HashSet::new();
        let (x, y) = point;

        for dx in 0..=limit {
            for dy in 0..=(limit - dx) {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let candidates = [
                    (x.saturating_sub(dx), y + dy),
                    (x + dx, y.saturating_sub(dy)),
                    (x + dx, y + dy),
                    (x.saturating_sub(dx), y.saturating_sub(dy)),
                ];
                for &(nx, ny) in &candidates {
                    if self.in_range((nx, ny)) {
                        neighbors.insert((nx, ny));
                    }
                }
            }
        }

        neighbors
    }

    fn in_range(&self, (x, y): (usize, usize)) -> bool {
        x < self.map.len() && y < self.map[0].len()
    }

    fn is_safe(&self, (x, y): (usize, usize)) -> bool {
        self.in_range((x, y)) && self.map[x][y] != '#'
    }
}

pub fn solve_a() {
    let map = Map::from_iter(read_lines("d20.txt", 24));
    map.find_cheats_at(2, 100);
}

pub fn solve_b() {
    let map = Map::from_iter(read_lines("d20.txt", 24));
    map.find_cheats_within(20, 100);
}
