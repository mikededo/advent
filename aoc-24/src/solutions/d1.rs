use std::collections::HashMap;

use crate::solutions::helpers::read_lines;

pub fn solve_a() {
    let (mut left, mut right) = read_lines("d1.txt").iter().fold(
        (Vec::new(), Vec::new()),
        |(mut left, mut right), line| {
            let values = line.split("   ").collect::<Vec<&str>>();
            // No need to check if the values are correct, as input is valid
            left.push(values[0].parse::<i32>().unwrap());
            right.push(values[1].parse::<i32>().unwrap());
            (left, right)
        },
    );

    left.sort();
    right.sort();

    let mut sum = 0;
    // Iterate and sum
    left.iter().enumerate().for_each(|(i, left_val)| {
        let right_val = right[i];
        sum += i32::abs(left_val - right_val);
    });
    println!("{}", sum);
}

pub fn solve_b() {
    let (positions, group): (Vec<i32>, HashMap<i32, i32>) = read_lines("d1.txt").iter().fold(
        (Vec::new(), HashMap::new()),
        |(mut positions, mut group), line| {
            let values = line.split("   ").collect::<Vec<&str>>();
            positions.push(values[0].parse::<i32>().unwrap());
            *group.entry(values[1].parse::<i32>().unwrap()).or_insert(0) += 1;
            (positions, group)
        },
    );

    println!(
        "{}",
        positions
            .iter()
            .fold(0, |acc, x| { acc + group.get(x).unwrap_or(&0) * x })
    );
}
