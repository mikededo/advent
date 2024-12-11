use std::collections::HashMap;

use utils::read_lines;

struct StoneCounter {
    cache: HashMap<(u64, u64), u64>,
}

impl StoneCounter {
    fn new() -> StoneCounter {
        StoneCounter {
            cache: HashMap::new(),
        }
    }

    fn count(&mut self, n: u64, blinks: u64) -> u64 {
        if blinks == 0 {
            return 1;
        }

        if let Some(c) = self.cache.get(&(n, blinks)) {
            return *c;
        }

        let mut count = 0;
        let digits = StoneCounter::count_digits(n);
        if n == 0 {
            count += self.count(1, blinks - 1);
        } else if digits % 2 == 0 {
            let exp = 10u64.pow(digits / 2);
            count += self.count(n / exp, blinks - 1) + self.count(n % exp, blinks - 1);
        } else {
            count += self.count(n * 2024, blinks - 1);
        }

        self.cache.insert((n, blinks), count);
        count
    }

    fn count_digits(mut n: u64) -> u32 {
        if n == 0 {
            return 1;
        }
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }
}

pub fn solve_a() {
    let mut counter = StoneCounter::new();
    let res: u64 = read_lines("d11.txt", 24)
        .into_iter()
        .next()
        .unwrap_or_else(|| unreachable!())
        .split_whitespace()
        .map(|s| match s.parse() {
            Ok(n) => counter.count(n, 25),
            Err(_) => 0,
        })
        .sum();

    println!("{res}");
}

pub fn solve_b() {
    let mut counter = StoneCounter::new();
    let res: u64 = read_lines("d11.txt", 24)
        .into_iter()
        .next()
        .unwrap_or_else(|| unreachable!())
        .split_whitespace()
        .map(|s| match s.parse() {
            Ok(n) => counter.count(n, 75),
            Err(_) => 0,
        })
        .sum();

    println!("{res}");
}
