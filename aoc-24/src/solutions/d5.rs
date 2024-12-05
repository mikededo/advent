use std::collections::{HashMap, HashSet};

use super::helpers::read_lines;

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
    read_lines("d5.txt").iter().for_each(|line| {
        if line.is_empty() {
            parse_updates = true;
            return;
        }

        if parse_updates {
            let values: Vec<u32> = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            res.push(is_rule_valid(&graph, &values));
        } else {
            let rule = line
                .split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
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
