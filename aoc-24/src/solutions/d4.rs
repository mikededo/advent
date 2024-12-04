use super::helpers::read_chars;

fn depth_letter(d: usize, l: &str) -> bool {
    match d {
        0 => l == "X",
        1 => l == "M",
        2 => l == "A",
        3 => l == "S",
        _ => false,
    }
}

fn get_cell(m: &[Vec<String>], y: usize, x: usize) -> Result<&String, usize> {
    let row = m.get(y);
    if row.is_none() {
        return Err(0);
    }
    let col = row.unwrap().get(x);
    if col.is_none() {
        return Err(0);
    }
    let cell = col.unwrap();
    Ok(cell)
}

fn v_find(c: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, down: bool) -> usize {
    let cell = match get_cell(c, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if down {
        v_find(c, (x, y + 1), depth + 1, true)
    } else if y > 0 {
        v_find(c, (x, y - 1), depth + 1, false)
    } else {
        0
    }
}

fn h_find(c: &Vec<Vec<String>>, (x, y): (usize, usize), depth: usize, right: bool) -> usize {
    let cell = match get_cell(c, y, x) {
        Ok(v) => v,
        Err(v) => return v,
    };

    let is_valid = depth_letter(depth, cell);
    // End cases
    if depth == 3 || !is_valid {
        return is_valid as usize;
    }

    if right {
        h_find(c, (x + 1, y), depth + 1, true)
    } else if x > 0 {
        h_find(c, (x - 1, y), depth + 1, false)
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

fn count_xmask(c: &Vec<Vec<String>>, (x, y): (usize, usize)) -> usize {
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
            .fold(0, |acc, (j, _)| acc + count_xmask(&res, (j, i)))
    });
    println!("{:?}", count);
}
