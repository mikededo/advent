use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};
use utils::read_bytes;

struct Garden<T: PartialEq + Copy + Display> {
    map: Vec<Vec<T>>,
}

impl<T: PartialEq + Copy + Display> Garden<T> {
    fn from_map(map: Vec<Vec<T>>) -> Self {
        Garden { map }
    }

    fn calculate_cost(&mut self) -> usize {
        let mut visited = HashSet::<(usize, usize)>::new();

        let rows = self.map.len();
        let cols = self.map[0].len();
        let mut cost: usize = 0;

        for (x, row) in self.map.iter().enumerate() {
            for (y, entry) in row.iter().enumerate() {
                if visited.contains(&(x, y)) {
                    continue;
                }

                visited.insert((x, y));

                let mut queue = VecDeque::new();
                queue.push_back((x, y));

                let mut edges = 0;
                let mut area = 0;
                while let Some((x, y)) = queue.pop_front() {
                    area += 1;
                    let directions = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ];

                    for (nx, ny) in directions {
                        let in_range = nx < cols && ny < rows;
                        if !in_range {
                            edges += 1;
                            continue;
                        }

                        let same = self.map[nx][ny] == *entry;
                        if same && !visited.contains(&(nx, ny)) {
                            let point = (nx, ny);
                            visited.insert(point);
                            queue.push_back(point);
                        } else if !same {
                            edges += 1;
                        }
                    }
                }
                cost += edges * area;
            }
        }

        cost
    }
}

pub fn solve_a() {
    let mut resolver = Garden::from_map(read_bytes("d12.txt", 24));
    println!("{:?}", resolver.calculate_cost());
}
