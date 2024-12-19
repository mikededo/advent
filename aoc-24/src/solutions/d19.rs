use std::collections::{HashMap, HashSet};

use utils::read_lines;

fn compute(towel: &str, designs: &HashSet<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = cache.get(towel) {
        return count;
    }

    let mut count = 0;
    for &design in designs {
        if towel == design {
            count += 1;
        }

        if towel.starts_with(design) {
            let next = towel.replacen(design, "", 1);
            count += compute(&next, designs, cache);
        }
    }

    cache.insert(towel.to_string(), count);
    count
}

pub fn solve_a() {
    let mut designs: HashSet<&str> = HashSet::new();
    let mut cache = HashMap::new();

    let res: usize = read_lines("d19.txt", 24)
        .iter()
        .enumerate()
        .filter_map(|(i, line)| match i {
            0 => {
                designs.extend(line.split(", "));
                None
            }
            1 => None,
            _ => match compute(line, &designs, &mut cache) {
                0 => None,
                count => Some(count),
            },
        })
        .count();

    println!("{:?}", res);
}

pub fn solve_b() {
    let mut designs: HashSet<&str> = HashSet::new();
    let mut cache = HashMap::new();

    let res: usize = read_lines("d19.txt", 24)
        .iter()
        .enumerate()
        .filter_map(|(i, line)| match i {
            0 => {
                designs.extend(line.split(", "));
                None
            }
            1 => None,
            _ => Some(compute(line, &designs, &mut cache)),
        })
        .sum();

    println!("{:?}", res);
}
