use std::{
    collections::HashMap,
    ops::{AddAssign, Index, IndexMut, Mul},
};

use utils::read_input_files;

#[derive(Debug, Default, Clone)]
struct Counter(HashMap<String, i64>);

impl Counter {
    fn from_elems<'a>(elems: impl IntoIterator<Item = &'a str>) -> Self {
        let mut counter = Self::default();

        for e in elems {
            counter[e.to_string()] += 1;
        }

        counter
    }
}

impl Index<String> for Counter {
    type Output = i64;

    fn index(&self, index: String) -> &Self::Output {
        self.0.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<String> for Counter {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        self.0.entry(index).or_default()
    }
}

impl AddAssign for Counter {
    fn add_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.0.into_iter() {
            self[k] += v;
        }
    }
}

impl Mul<i64> for Counter {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0.into_iter().map(|(k, v)| (k, v * rhs)).collect())
    }
}

fn parse_input(input: &str) -> HashMap<String, Counter> {
    let mut rules = HashMap::new();

    for line in input.trim().lines() {
        let (head, tail) = line.split_once(':').unwrap();

        let rule_body = Counter::from_elems(tail.split(','));

        rules.insert(head.to_string(), rule_body);
    }

    rules
}

fn simulate<'a>(
    rules: &HashMap<String, Counter>,
    initial_pop: impl IntoIterator<Item = &'a str>,
    days: usize,
) -> i64 {
    let mut curr = Counter::from_elems(initial_pop);

    for _ in 0..days {
        let mut next = Counter::default();

        for (generation, count) in curr.0.into_iter() {
            next += rules[&generation].clone() * count;
        }

        curr = next;
    }

    curr.0.values().sum()
}

fn part1(input: &str) -> i64 {
    let rules = parse_input(input);
    simulate(&rules, ["A"], 4)
}

fn part2(input: &str) -> i64 {
    let rules = parse_input(input);
    simulate(&rules, ["Z"], 10)
}

fn part3(input: &str) -> i64 {
    let rules = parse_input(input);

    let generation_types = rules.keys();

    let values_for_initial_pops: Vec<_> = generation_types
        .map(|generation| simulate(&rules, [&generation[..]], 20))
        .collect();

    let min = values_for_initial_pops.iter().min().unwrap();
    let max = values_for_initial_pops.iter().max().unwrap();

    max - min
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
