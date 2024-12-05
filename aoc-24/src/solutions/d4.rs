use utils::read_chars;

fn depth_letter(depth: usize, letter: &str) -> bool {
    match depth {
        0 => letter == "X",
        1 => letter == "M",
        2 => letter == "A",
        3 => letter == "S",
        _ => false,
    }
}

fn get_cell(matrix: &[Vec<String>], y: usize, x: usize) -> Result<&String, usize> {
    match matrix.get(y) {
        Some(row) => match row.get(x) {
            Some(cell) => Ok(cell),
            None => Err(0),
        },
        None => Err(0),
    }
}

fn v_find(matrix: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, down: bool) -> usize {
    let cell = match get_cell(matrix, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if down {
        v_find(matrix, (x, y + 1), depth + 1, true)
    } else if y > 0 {
        v_find(matrix, (x, y - 1), depth + 1, false)
    } else {
        0
    }
}

fn h_find(matrix: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, right: bool) -> usize {
    let cell = match get_cell(matrix, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if right {
        h_find(matrix, (x + 1, y), depth + 1, true)
    } else if x > 0 {
        h_find(matrix, (x - 1, y), depth + 1, false)
    } else {
        0
    }
}

fn l_find(c: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, up: bool) -> usize {
    let cell = match get_cell(c, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if up {
        if y == 0 || x == 0 {
            0
        } else {
            l_find(c, (x - 1, y - 1), depth + 1, true)
        }
    } else if x > 0 {
        l_find(c, (x - 1, y + 1), depth + 1, false)
    } else {
        0
    }
}

fn r_find(c: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, up: bool) -> usize {
    let cell = match get_cell(c, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if up {
        if y > 0 {
            r_find(c, (x + 1, y - 1), depth + 1, true)
        } else {
            0
        }
    } else {
        r_find(c, (x + 1, y + 1), depth + 1, false)
    }
}

fn count_xmas(c: &Vec<Vec<String>>, (x, y): (usize, usize)) -> usize {
    h_find(c, (x, y), 0, true)
        + h_find(c, (x, y), 0, false)
        + v_find(c, (x, y), 0, true)
        + v_find(c, (x, y), 0, false)
        + l_find(c, (x, y), 0, true)
        + l_find(c, (x, y), 0, false)
        + r_find(c, (x, y), 0, true)
        + r_find(c, (x, y), 0, false)
}

pub fn solve_a() {
    let res = read_chars("d4.txt", 24);
    let count = res.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, _)| acc + count_xmas(&res, (j, i)))
    });
    println!("{:?}", count);
}

fn cell_matches(matrix: &[Vec<String>], (x, y): (usize, usize), expected: &str) -> bool {
    get_cell(matrix, y, x).map_or(false, |cell| cell == expected)
}

fn check_case(
    matrix: &[Vec<String>],
    (x, y): (usize, usize),
    [top_left, top_right, bottom_left, bottom_right]: [&str; 4],
) -> bool {
    cell_matches(matrix, (x - 1, y - 1), top_left)
        && cell_matches(matrix, (x + 1, y - 1), top_right)
        && cell_matches(matrix, (x - 1, y + 1), bottom_left)
        && cell_matches(matrix, (x + 1, y + 1), bottom_right)
}

// Possible cases
// 1.    2.    3.    4.
// M.M   S.S   M.S   S.M
// .A.   .A.   .A.   .A.
// S.S   M.M   M.S   S.M
const CASES: [[&str; 4]; 4] = [
    ["M", "M", "S", "S"],
    ["M", "S", "M", "S"],
    ["S", "M", "S", "M"],
    ["S", "S", "M", "M"],
];
pub fn solve_b() {
    let res = read_chars("d4.txt", 24);
    let count = res.iter().enumerate().fold(0, |acc, (i, row)| {
        if i == 0 || i == res.len() - 1 {
            return acc;
        }

        acc + row.iter().enumerate().fold(0, |acc, (j, cell)| {
            if j == 0 || j == row.len() - 1 || cell != "A" {
                return acc;
            }

            acc + (CASES.iter().any(|comb| check_case(&res, (j, i), *comb))) as usize
        })
    });
    println!("{:?}", count);
}
