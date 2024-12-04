use super::helpers::read_chars;

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
    let res = read_chars("d4.txt");

    res.iter().for_each(|row| {
        row.iter().for_each(|c| {
            print!("{c} ");
        });
        println!();
    });
    let count = res.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, _)| acc + count_xmas(&res, (j, i)))
    });
    println!("{:?}", count);
}
