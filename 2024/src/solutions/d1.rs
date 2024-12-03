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
