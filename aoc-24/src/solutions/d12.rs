use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};
use utils::read_bytes;

struct Garden<T: PartialEq + Copy + Display> {
    map: Vec<Vec<T>>,
    cols: usize,
    rows: usize,
}

impl<T: PartialEq + Copy + Display> Garden<T> {
    fn from_map(map: Vec<Vec<T>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        Garden { map, rows, cols }
    }

    /// Performs iterative flood fill algorithm (very interesting) - found as alternatives to BFS
    /// and DFS
    ///
    /// <https://en.wikipedia.org/wiki/Flood_fill>
    fn calculate_cost(&mut self, sides: bool) -> usize {
        let mut visited = HashSet::<(usize, usize)>::new();
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
                    if sides {
                        edges += self.count_point_corners(x, y);
                    }

                    for (nx, ny) in Garden::<T>::point_sides(x, y) {
                        if !self.in_range(nx, ny) {
                            edges += if sides { 0 } else { 1 };
                            continue;
                        }

                        let eq_val = self.map[nx][ny] == *entry;
                        let point = (nx, ny);
                        if eq_val && !visited.contains(&point) {
                            visited.insert(point);
                            queue.push_back(point);
                        } else if !eq_val && !sides {
                            edges += 1;
                        }
                    }
                }

                cost += edges * area;
            }
        }

        cost
    }

    fn count_point_corners(&self, ix: usize, iy: usize) -> usize {
        let mut count = 0;
        let val = self.map[ix][iy];

        let [top, right, bottom, left] = Garden::<T>::point_sides(ix, iy)
            .map(|(x, y)| !self.in_range(x, y) || self.map[x][y] != val);
        count += (top && right) as usize;
        count += (top && left) as usize;
        count += (bottom && right) as usize;
        count += (bottom && left) as usize;

        let [tl, tr, bl, br] = Garden::<T>::point_diagonals(ix, iy)
            .map(|(x, y)| !self.in_range(x, y) || self.map[x][y] != val);
        count += (tl && !top && !left) as usize;
        count += (tr && !top && !right) as usize;
        count += (bl && !bottom && !left) as usize;
        count += (br && !bottom && !right) as usize;

        count
    }

    fn in_range(&self, x: usize, y: usize) -> bool {
        x < self.rows && y < self.cols
    }

    /// [top, right, bottom, left]
    fn point_sides(x: usize, y: usize) -> [(usize, usize); 4] {
        [
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
        ]
    }

    /// [top-left, top-right, bottom-left, bottom-right]
    fn point_diagonals(x: usize, y: usize) -> [(usize, usize); 4] {
        [
            (x.wrapping_sub(1), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y + 1),
            (x + 1, y.wrapping_sub(1)),
            (x + 1, y + 1),
        ]
    }
}

pub fn solve_a() {
    let mut resolver = Garden::from_map(read_bytes("d12.txt", 24));
    println!("{:?}", resolver.calculate_cost(false));
}

pub fn solve_b() {
    let mut resolver = Garden::from_map(read_bytes("d12.txt", 24));
    println!("{:?}", resolver.calculate_cost(true));
}
