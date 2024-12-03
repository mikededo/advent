use std::collections::HashMap;

use regex::Regex;

use super::helpers::read_lines;

type Graph = HashMap<String, (String, String)>;
trait GraphIter {
    fn left(&self, val: &str) -> &str;
    fn right(&self, val: &str) -> &str;
}

impl GraphIter for Graph {
    fn left(&self, val: &str) -> &str {
        if let Some((left, _)) = self.get(val) {
            left
        } else {
            panic!("Key ({val}) not found in graph");
        }
    }

    fn right(&self, val: &str) -> &str {
        if let Some((_, right)) = self.get(val) {
            right
        } else {
            panic!("Key ({val}) not found in graph");
        }
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

struct Data {
    ins: Vec<char>,
    start_points: Vec<String>,
    graph: HashMap<String, (String, String)>,
}

fn parse_data() -> Data {
    let entry_regex: Regex =
        Regex::new(r"(?P<key>\w+) = \((?P<left>\w+), (?P<right>\w+)\)").unwrap();

    let lines = read_lines("day8.txt");
    let ins: Vec<_> = lines.first().unwrap().chars().collect();
    let mut start_points: Vec<String> = Vec::new();

    let mut graph: Graph = Graph::new();
    lines.iter().skip(2).for_each(|s| {
        if let Some(caps) = entry_regex.captures(s) {
            let key = caps.name("key").unwrap().as_str().to_owned();
            let left = caps.name("left").unwrap().as_str().to_owned();
            let right = caps.name("right").unwrap().as_str().to_owned();

            if key.ends_with('A') {
                start_points.push(key.clone());
            }
            graph.insert(key, (left, right));
        }
    });

    Data {
        ins,
        start_points,
        graph,
    }
}

fn find_steps(
    start: &str,
    ins: &Vec<char>,
    graph: &HashMap<String, (String, String)>,
    partial: bool,
) -> usize {
    let mut steps = 0;
    let mut key = start;

    while if partial {
        !key.ends_with('Z')
    } else {
        key != END
    } {
        let next = ins[steps % ins.len()];
        match next {
            'L' => key = graph.left(key),
            'R' => key = graph.right(key),
            x => panic!("Illegal step found => ({x})"),
        };
        steps += 1;
    }
    steps
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve_a() {
    let Data {
        ins,
        start_points: _,
        graph,
    } = parse_data();

    println!("{}", find_steps(START, &ins, &graph, false));
}

pub fn solve_b() {
    let Data {
        ins,
        start_points,
        graph,
    } = parse_data();

    let points = start_points
        .iter()
        .map(|sp| find_steps(sp, &ins, &graph, true));
    println!("{}", points.reduce(lcm).unwrap());
}
