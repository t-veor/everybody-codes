use std::collections::HashSet;

use utils::{grid::OrthoDir, read_input_files};

fn matches_overlapping(needle: &str, mut haystack: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let mut start = 0;

    while let Some(i) = haystack.find(needle) {
        matches.push(start + i);

        let mut j = i + 1;
        while !haystack.is_char_boundary(j) {
            j += 1;
        }

        start += j;
        haystack = &haystack[j..]
    }

    matches
}

fn part1(input: &str) -> usize {
    let (words, haystack) = input.trim().split_once("\n\n").unwrap();
    let needles = words.strip_prefix("WORDS:").unwrap().split(',');

    needles
        .map(|needle| matches_overlapping(needle, haystack).len())
        .sum()
}

fn part2(input: &str) -> usize {
    let (words, haystack) = input.trim().split_once("\n\n").unwrap();

    let needles = words.strip_prefix("WORDS:").unwrap().split(',');
    let mut is_runic_letter = vec![false; haystack.len()];

    for needle in needles {
        let needle_rev = needle.chars().rev().collect::<String>();

        for n in [needle, &needle_rev] {
            for i in matches_overlapping(n, haystack) {
                is_runic_letter[i..i + n.len()].fill(true);
            }
        }
    }

    is_runic_letter.iter().filter(|&&x| x).count()
}

struct WordSearch {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    fn new(haystack: &str) -> Self {
        let grid = haystack
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();

        Self {
            width,
            height,
            grid,
        }
    }

    fn wrap_coords(&self, coords: (isize, isize)) -> (isize, isize) {
        let (x, y) = coords;
        (x.rem_euclid(self.width as isize), y)
    }

    fn get(&self, coords: (isize, isize)) -> Option<char> {
        let (x, y) = self.wrap_coords(coords);
        if y < 0 {
            None
        } else {
            self.grid
                .get(y as usize)
                .and_then(|line| line.get(x as usize))
                .copied()
        }
    }

    fn get_word(
        &self,
        start: (isize, isize),
        dir: OrthoDir,
        len: usize,
    ) -> (String, Vec<(isize, isize)>) {
        let mut coords = start;

        let mut word = String::new();
        let mut word_chars = Vec::new();

        for _ in 0..len {
            coords = self.wrap_coords(coords);

            let Some(char) = self.get(coords) else { break };
            word.push(char);
            word_chars.push(coords);

            coords = dir.step(coords);
        }

        (word, word_chars)
    }

    fn all_coords(&self) -> impl Iterator<Item = (isize, isize)> {
        let (width, height) = (self.width, self.height);
        (0..height).flat_map(move |y| (0..width).map(move |x| (x as isize, y as isize)))
    }
}

fn part3(input: &str) -> usize {
    // It's making us do a word search, wow
    let (words, haystack) = input.trim().split_once("\n\n").unwrap();
    let needles = words
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();
    let wordsearch = WordSearch::new(haystack);

    let mut is_runic = HashSet::new();

    for &needle in needles.iter() {
        for coords in wordsearch.all_coords() {
            // Check the first character before trying to step
            if needle.chars().next() != wordsearch.get(coords) {
                continue;
            }

            for dir in OrthoDir::ALL {
                let (word, word_coords) = wordsearch.get_word(coords, dir, needle.len());

                if needle == word {
                    for x in word_coords {
                        if x == (2, 0) {
                            dbg!(needle);
                        }
                        is_runic.insert(x);
                    }
                }
            }
        }
    }

    is_runic.len()
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
