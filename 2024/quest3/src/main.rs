use std::collections::HashSet;

use utils::{
    grid::{DiagDir, OrthoDir},
    read_input_files,
};

// Eh, this could probably have been implemented more efficiently as a DFS, but
// this is more than fast enough
fn dig_out<F, I>(input: &str, adjacent: F) -> i64
where
    F: Fn((isize, isize)) -> I,
    I: Iterator<Item = (isize, isize)>,
{
    let mut to_dig: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut can_be_dug_next = HashSet::new();

    let mut dug_tiles = 0;
    while !to_dig.is_empty() {
        for &tile in to_dig.iter() {
            dug_tiles += 1;

            if adjacent(tile).all(|adj_tile| to_dig.contains(&adj_tile)) {
                can_be_dug_next.insert(tile);
            }
        }

        to_dig.clear();
        std::mem::swap(&mut to_dig, &mut can_be_dug_next);
    }

    dug_tiles
}

fn part1(input: &str) -> i64 {
    dig_out(input, |tile| {
        OrthoDir::ALL.iter().map(move |dir| dir.step(tile))
    })
}

fn part2(input: &str) -> i64 {
    part1(input)
}

fn part3(input: &str) -> i64 {
    dig_out(input, |tile| {
        DiagDir::ALL.iter().map(move |dir| dir.step(tile))
    })
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
