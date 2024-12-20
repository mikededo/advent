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
    fn find_cheats(&self) {
        let min_path = self.find_shortest();
        let min_dist = min_path.get(&self.end).unwrap();
        let mut rest = BTreeMap::new();

        for (point, steps) in &min_path {
            for adj in n_adjacent(*point, 2) {
                if let Some(dist) = min_path.get(&adj) {
                    // This 2 drove me mad! We need to also add the cheat!
                    let final_dist = min_dist - dist + steps + 2;
                    if final_dist < *min_dist && *min_dist - final_dist >= 100 {
                        rest.entry(final_dist).and_modify(|c| *c += 1).or_insert(1);
                    }
                }
            }
        }

        println!("{:?}", rest.values().sum::<usize>());
    }

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

    fn is_safe(&self, (x, y): (usize, usize)) -> bool {
        x < self.map.len() && y < self.map[0].len() && self.map[x][y] != '#'
    }
}

pub fn solve_a() {
    let map = Map::from_iter(read_lines("d20.txt", 24));
    map.find_cheats();
}
