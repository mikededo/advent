use std::collections::{HashMap, HashSet};

use utils::read_lines;

pub fn is_rule_valid(graph: &HashMap<u32, HashSet<u32>>, rule: &[u32]) -> u32 {
    let empty_rules = HashSet::new();
    let result = rule[rule.len() / 2];
    let valid = rule.iter().enumerate().all(|(i, prev)| {
        rule[i + 1..]
            .iter()
            // Check if the number that's before (prev) should be after next
            .all(|next| !graph.get(next).unwrap_or(&empty_rules).contains(prev))
    });

    if valid {
        result
    } else {
        0
    }
}

pub fn solve_a() {
    let mut parse_updates = false;
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();

    let mut res: Vec<u32> = Vec::new();
    read_lines("d5.txt", 24).iter().for_each(|line| {
        if line.is_empty() {
            parse_updates = true;
            return;
        }

        if parse_updates {
            let values: Vec<u32> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            res.push(is_rule_valid(&graph, &values));
        } else {
            let rule: Vec<u32> = line.split('|').filter_map(|n| n.parse().ok()).collect();
            graph
                .entry(rule[0])
                .and_modify(|s| {
                    s.insert(rule[1]);
                })
                .or_insert(HashSet::from([rule[1]]));
        }
    });

    println!("{:?}", res.iter().sum::<u32>());
}

fn can_be_first(graph: &HashMap<u32, HashSet<u32>>, value: u32, rest: &[&u32]) -> bool {
    let empty_rules = HashSet::new();
    let rules = graph.get(&value).unwrap_or(&empty_rules);
    rest.iter().all(|n| rules.contains(*n))
}

fn validate_rule(graph: &HashMap<u32, HashSet<u32>>, rule: &[u32]) -> u32 {
    let mut corrected: Vec<u32> = Vec::new();
    let mut stack = rule.to_vec();
    let mut i = 0;
    while !stack.is_empty() {
        if i == stack.len() && stack.len() > 1 {
            i = 0;
            continue;
        } else if stack.len() == 1 {
            corrected.push(stack[0]);
            stack.pop();
            continue;
        }

        // Check if the current number can be first
        let rest: Vec<&u32> = stack[..i].iter().chain(&stack[i + 1..]).collect();
        if can_be_first(graph, stack[i], &rest) {
            corrected.push(stack[i]);
            stack.remove(i);
        } else {
            i += 1;
        }
    }

    corrected[corrected.len() / 2]
}

pub fn solve_b() {
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut parse_updates = false;
    let mut res = Vec::new();

    for line in read_lines("d5.txt", 24) {
        if line.is_empty() {
            parse_updates = true;
            continue;
        }

        if parse_updates {
            let values: Vec<u32> = line.split(',').filter_map(|n| n.parse().ok()).collect();
            if is_rule_valid(&graph, &values) == 0 {
                res.push(validate_rule(&graph, &values));
            }
        } else {
            let rule: Vec<u32> = line.split('|').filter_map(|n| n.parse().ok()).collect();
            graph
                .entry(rule[0])
                .and_modify(|s| {
                    s.insert(rule[1]);
                })
                .or_insert(HashSet::from([rule[1]]));
        }
    }

    println!("{:?}", res.iter().sum::<u32>());
}
