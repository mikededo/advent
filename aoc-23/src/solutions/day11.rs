use std::collections::HashSet;

use utils::read_lines;

fn expanded_distance(
    p1: [usize; 2],
    p2: [usize; 2],
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
) -> usize {
    let (r1, c1) = (p1[0], p1[1]);
    let (r2, c2) = (p2[0], p2[1]);

    let row_dist = r1.abs_diff(r2)
        + empty_rows
            .iter()
            .filter(|&&r| r > r1.min(r2) && r < r1.max(r2))
            .count();
    let col_dist = c1.abs_diff(c2)
        + empty_cols
            .iter()
            .filter(|&&c| c > c1.min(c2) && c < c1.max(c2))
            .count();

    row_dist + col_dist
}

pub fn solve_a() {
    let mut mat: Vec<Vec<char>> = Vec::new();
    let mut galaxies: Vec<[usize; 2]> = Vec::new();

    read_lines("day11.txt", 23)
        .iter()
        .enumerate()
        .for_each(|(i, l)| {
            mat.push(vec![]);
            l.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    galaxies.push([i, j]);
                }
                mat[i].push(c);
            })
        });

    let empty_rows: HashSet<usize> = (0..mat.len())
        .filter(|&i| mat[i].iter().all(|&c| c == '.'))
        .collect();
    let empty_cols: HashSet<usize> = (0..mat[0].len())
        .filter(|&j| mat.iter().all(|row| row[j] == '.'))
        .collect();

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total += expanded_distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols);
        }
    }

    println!("{}", total);
}
