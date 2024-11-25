use std::collections::{BTreeMap, HashMap};

use utils::read_input_files;

struct DAG {
    adjacency: HashMap<String, Vec<String>>,
}

impl DAG {
    fn new(input: &str) -> Self {
        let mut adjacency = HashMap::new();

        for line in input.trim().lines() {
            let (parent, children) = line.split_once(':').unwrap();
            let children = children.split(',').map(|s| s.to_string()).collect();

            adjacency.insert(parent.to_string(), children);
        }

        Self { adjacency }
    }
}

fn visit_all_fruits(dag: &DAG) -> BTreeMap<i32, Vec<Vec<&str>>> {
    fn visitor<'a>(
        dag: &'a DAG,
        node: &'a str,
        path: &mut Vec<&'a str>,
        depth: i32,
        visit: &mut impl FnMut(&[&'a str], i32),
    ) {
        if path.contains(&node) {
            return;
        }

        path.push(node);

        if node == "@" {
            visit(path, depth);
        } else {
            let children = dag
                .adjacency
                .get(node)
                .map(|x| x.as_slice())
                .unwrap_or_default();

            for child in children {
                visitor(dag, &child, path, depth + 1, visit);
            }
        }

        path.pop();
    }

    let mut fruits_by_depth = BTreeMap::<i32, Vec<Vec<&str>>>::new();

    visitor(dag, "RR", &mut Vec::new(), 0, &mut |path, depth| {
        let entry = fruits_by_depth.entry(depth);
        entry.or_default().push(path.to_vec());
    });

    fruits_by_depth
}

fn part1(input: &str) -> String {
    let tree = DAG::new(input);
    let fruits_by_depth = visit_all_fruits(&tree);

    for (_, paths) in fruits_by_depth.iter() {
        if paths.len() == 1 {
            return paths[0].join("");
        }
    }

    panic!("No depth with only one fruit found")
}

fn part2(input: &str) -> String {
    let tree = DAG::new(input);
    let fruits_by_depth = visit_all_fruits(&tree);

    for (_, paths) in fruits_by_depth.iter() {
        if paths.len() == 1 {
            return paths[0].iter().map(|s| s.chars().next().unwrap()).collect();
        }
    }

    panic!("No depth with only one fruit found")
}

fn part3(input: &str) -> String {
    part2(input)
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
