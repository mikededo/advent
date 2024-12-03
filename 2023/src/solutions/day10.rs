use super::helpers::read_lines;

fn find_start_points(mat: &[Vec<char>]) -> (usize, usize) {
    // Find starting points
    let (sx, sy) = mat
        .iter()
        .enumerate()
        .fold(Option::<(usize, usize)>::None, |res, (i, l)| match res {
            Some(_) => res,
            None => l.iter().position(|c| *c == 'S').map(|pos| (pos, i)),
        })
        .unwrap_or_else(|| panic!("Could not find char S"));
    (sx, sy)
}

pub fn find_next_suitable(mat: &[Vec<char>], (y, x): (usize, usize)) -> Option<(usize, usize)> {
    // No need to check for diagonals
    if mat.get(y + 1).is_some()
        && mat[y + 1]
            .get(x)
            .is_some_and(|c| !['.', '-', '7', 'F'].contains(c))
    {
        Some((y + 1, x))
    } else if mat[y]
        .get(x - 1)
        .is_some_and(|c| !['.', '|', 'J', '7'].contains(c))
    {
        Some((y, x - 1))
    } else if mat[y]
        .get(x + 1)
        .is_some_and(|c| !['.', '|', 'F', 'L'].contains(c))
    {
        Some((y, x + 1))
    } else if mat.get(y - 1).is_some()
        && mat[y - 1]
            .get(x)
            .is_some_and(|c| !['.', '-', 'L', 'J'].contains(c))
    {
        Some((y - 1, x))
    } else {
        None
    }
}

pub fn calc_path(mat: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visited: Vec<(usize, usize)> = vec![
        (y, x),
        find_next_suitable(mat, (y, x)).unwrap_or_else(|| panic!("Unable to find next point")),
    ];

    loop {
        let (cy, cx) = visited[visited.len() - 1];
        let (py, px) = visited[visited.len() - 2];
        let c = mat[cy][cx];

        let next = match c {
            '-' => (cy, if cx > px { cx + 1 } else { cx - 1 }),
            '|' => (if cy > py { cy + 1 } else { cy - 1 }, cx),
            'L' => {
                if cx < px {
                    (cy - 1, cx)
                } else {
                    (cy, cx + 1)
                }
            }
            'J' => {
                if cx > px {
                    (cy - 1, cx)
                } else {
                    (cy, cx - 1)
                }
            }
            '7' => {
                if cx > px {
                    (cy + 1, cx)
                } else {
                    (cy, cx - 1)
                }
            }
            'F' => {
                if cx < px {
                    (cy + 1, cx)
                } else {
                    (cy, cx + 1)
                }
            }
            'S' => {
                return visited;
            }
            _ => panic!("Illegal character ({c}) at ({cy}, {cx})"),
        };
        visited.push(next);
    }
}

fn points_to_edges(path: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut edges = vec![path[0]];
    let mut on_x = path[0].0 != path[1].0;
    let mut on_y = path[0].1 != path[1].1;

    for i in 2..path.len() {
        if on_x && path[i].0 == path[i - 1].0 {
            on_x = false;
            on_y = true;
            edges.push((path[i].0, path[i - 1].1));
        } else if on_y && path[i].1 == path[i - 1].1 {
            on_x = true;
            on_y = false;
            edges.push((path[i - 1].0, path[i].1));
        }
    }

    edges.push(path[0]);

    edges
}

pub fn solve_a() {
    let mut mat: Vec<Vec<char>> = Vec::new();
    read_lines("day10.txt")
        .iter()
        .for_each(|l| mat.push(l.chars().collect::<Vec<char>>()));

    let (sx, sy) = find_start_points(&mat);

    println!("{}", (calc_path(&mat, sx, sy).len() - 1) / 2);
}

pub fn solve_b() {
    let mut mat: Vec<Vec<char>> = Vec::new();
    read_lines("day10.txt")
        .iter()
        .for_each(|l| mat.push(l.chars().collect::<Vec<char>>()));

    let (sx, sy) = find_start_points(&mat);

    let path = calc_path(&mat, sx, sy);
    let edges = points_to_edges(&path);
    let surface = edges
        .windows(2)
        .map(|w| (w[0].0 * w[1].1) as f32 - (w[0].1 * w[1].0) as f32)
        .sum::<f32>()
        .abs()
        / 2f32;

    println!("{}", (surface - (path.len() as f32 / 2f32) + 1f32).ceil());
}
