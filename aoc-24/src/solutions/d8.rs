use std::collections::{HashMap, HashSet};

use utils::read_chars_with;

fn generate_points(p1: (i32, i32), p2: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    ((p1.0 - dx, p1.1 - dy), (p2.0 + dx, p2.1 + dy))
}

pub fn solve_a() {
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let map = read_chars_with("d8.txt", 24, |ch, (row, col)| {
        if ch == '.' {
            return;
        }
        let pair = (row as i32, col as i32);
        antennas.entry(ch).or_default().insert(pair);
    });

    let map_range = (map.len() as i32, map[0].len() as i32);
    let in_range =
        |&(row, col): &(i32, i32)| row >= 0 && col >= 0 && row < map_range.0 && col < map_range.1;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for values in antennas.values() {
        let iter: Vec<(i32, i32)> = values.iter().cloned().collect();
        iter.iter().enumerate().for_each(|(i, curr)| {
            iter[i + 1..].iter().for_each(|next| {
                let (p1, p2) = generate_points(*curr, *next);
                if in_range(&p1) {
                    antinodes.insert(p1);
                }
                if in_range(&p2) {
                    antinodes.insert(p2);
                }
            });
        });
    }

    println!("{}", antinodes.len());
}

pub fn solve_b() {
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let map = read_chars_with("d8.txt", 24, |ch, (row, col)| {
        if ch == '.' {
            return;
        }
        let pair = (row as i32, col as i32);
        antennas.entry(ch).or_default().insert(pair);
    });

    let map_range = (map.len() as i32, map[0].len() as i32);
    let in_range =
        |&(row, col): &(i32, i32)| row >= 0 && col >= 0 && row < map_range.0 && col < map_range.1;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for values in antennas.values() {
        let iter: Vec<(i32, i32)> = values.iter().cloned().collect();
        // All antennas with at least two of their type
        if iter.len() > 1 {
            antinodes.extend(&iter);
        } else {
            continue;
        }

        iter.iter().enumerate().for_each(|(i, curr)| {
            iter[i + 1..].iter().for_each(|next| {
                let delta_x = next.0 - curr.0;
                let delta_y = next.1 - curr.1;

                let mut point = (curr.0 - delta_x, curr.1 - delta_y);
                while in_range(&point) {
                    antinodes.insert(point);
                    point = (point.0 - delta_x, point.1 - delta_y);
                }

                point = (next.0 + delta_x, next.1 + delta_y);
                while in_range(&point) {
                    antinodes.insert(point);
                    point = (point.0 + delta_x, point.1 + delta_y);
                }
            });
        });
    }

    println!("\n{}", antinodes.len());
}
