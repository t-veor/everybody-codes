use std::collections::HashMap;

use utils::{cycle_detection::brent, read_input_files};

fn dance_round(columns: &mut Vec<Vec<i64>>, clapper_col: usize) {
    let absorption_col = (clapper_col + 1) % columns.len();

    let clapper = columns[clapper_col].remove(0);

    let n = ((clapper - 1) as usize) % (2 * columns[absorption_col].len());
    let insert_pos = n.min(2 * columns[absorption_col].len() - n);
    columns[absorption_col].insert(insert_pos, clapper);
}

fn read_column_front(columns: &[Vec<i64>]) -> i64 {
    columns
        .iter()
        .map(|col| col.first().unwrap().to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn transpose(rows: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut columns = Vec::new();

    for i in 0..rows[0].len() {
        let mut col = Vec::new();
        for r in rows {
            col.push(r[i]);
        }
        columns.push(col);
    }

    columns
}

fn read_columns(input: &str) -> Vec<Vec<i64>> {
    let rows: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    transpose(&rows)
}

fn part1(input: &str) -> i64 {
    let mut columns = read_columns(input);
    let num_cols = columns.len();

    for i in 0..10 {
        dance_round(&mut columns, i % num_cols);
    }

    read_column_front(&columns)
}

fn part2(input: &str) -> i64 {
    let mut seen_times = HashMap::<i64, i64>::new();

    let mut columns = read_columns(input);
    let num_columns = columns.len();

    for i in 0.. {
        dance_round(&mut columns, i % num_columns);
        let number = read_column_front(&columns);

        let entry = seen_times.entry(number).or_default();
        *entry += 1;
        if *entry >= 2024 {
            return (i + 1) as i64 * number;
        }
    }

    unreachable!()
}

fn part3(input: &str) -> i64 {
    let mut columns = read_columns(input);
    let num_columns = columns.len();

    let cycle_info = brent(columns.clone(), |mut columns| {
        for i in 0..columns.len() {
            dance_round(&mut columns, i);
        }
        columns
    });

    let num_rounds_needed = (cycle_info.start + cycle_info.length) as usize * num_columns;

    let mut highest_seen = 0;
    for i in 0..num_rounds_needed {
        dance_round(&mut columns, i % num_columns);
        let number = read_column_front(&columns);

        if number > highest_seen {
            highest_seen = number;
        }
    }

    highest_seen
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
