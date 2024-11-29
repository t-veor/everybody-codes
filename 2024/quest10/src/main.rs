use utils::read_input_files;

fn try_find_shared_symbol(row: &[char], col: &[char]) -> Option<char> {
    for &r in row {
        for &c in col {
            if r != '?' && r == c {
                return Some(r);
            }
        }
    }

    None
}

fn runic_word(grid: &[Vec<char>]) -> String {
    let rows: Vec<Vec<char>> = grid[2..6]
        .iter()
        .map(|row| row[0..2].iter().chain(row[6..8].iter()).copied().collect())
        .collect();
    let columns: Vec<Vec<char>> = (2..6)
        .map(|i| {
            grid[0..2]
                .iter()
                .chain(grid[6..8].iter())
                .map(|row| row[i])
                .collect()
        })
        .collect();

    let mut word = String::new();

    for r in 0..4 {
        for c in 0..4 {
            let char = try_find_shared_symbol(&rows[r], &columns[c]).unwrap();
            word.push(char);
        }
    }

    word
}

fn part1(input: &str) -> String {
    let grid: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    runic_word(&grid)
}

fn runic_word_power(word: &str) -> u32 {
    word.chars()
        .enumerate()
        .map(|(i, c)| {
            let base = (c as u32) - ('A' as u32) + 1;
            base * (i as u32 + 1)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let grids: Vec<Vec<Vec<char>>> = input
        .trim()
        .split("\n\n")
        .flat_map(|row_of_blocks| {
            let internal_rows: Vec<Vec<&str>> = row_of_blocks
                .lines()
                .map(|line| line.split_whitespace().collect())
                .collect();

            (0..internal_rows[0].len()).map(move |i| {
                internal_rows
                    .iter()
                    .map(|row| row[i].chars().collect())
                    .collect()
            })
        })
        .collect();

    grids
        .iter()
        .map(|grid| {
            let word = runic_word(grid);
            runic_word_power(&word)
        })
        .sum()
}

struct Wall {
    grid: Vec<Vec<char>>,
    width_blocks: usize,
    height_blocks: usize,
}

impl Wall {
    fn get_grid_at(&self, x: usize, y: usize) -> Vec<Vec<char>> {
        let base_row = y * 6;
        let base_col = x * 6;

        (base_row..base_row + 8)
            .map(|r| (base_col..base_col + 8).map(|c| self.grid[r][c]).collect())
            .collect()
    }

    fn try_solve_at(&mut self, x: usize, y: usize) -> (Option<String>, bool) {
        #[derive(Debug, Clone, Copy)]
        enum Specifier {
            Row(usize, usize),
            Col(usize, usize),
        }

        let grid = self.get_grid_at(x, y);

        let mut rows: Vec<Vec<char>> = grid[2..6]
            .iter()
            .map(|row| row[0..2].iter().chain(row[6..8].iter()).copied().collect())
            .collect();
        let mut columns: Vec<Vec<char>> = (2..6)
            .map(|i| {
                grid[0..2]
                    .iter()
                    .chain(grid[6..8].iter())
                    .map(|row| row[i])
                    .collect()
            })
            .collect();

        // Quick sanity check
        for row in rows.iter() {
            if row.iter().filter(|&&x| x == '?').count() > 1 {
                // row contains more than 1 ?, bail
                return (None, false);
            }
        }
        for col in columns.iter() {
            if col.iter().filter(|&&x| x == '?').count() > 1 {
                // column contains more than 1 ?, bail
                return (None, false);
            }
        }

        let mut empty = vec![vec![true; 4]; 4];
        let mut word = Vec::new();
        let mut used_chars = Vec::new();
        let mut qmark_replacements = Vec::new();

        loop {
            let mut mutated_this_loop = false;

            // Solve as much as possible.
            for r in 0..4 {
                for c in 0..4 {
                    if !empty[r][c] {
                        continue;
                    }

                    if let Some(char) = try_find_shared_symbol(&rows[r], &columns[c]) {
                        word.push(((r, c), char));
                        used_chars.push(char);
                        empty[r][c] = false;
                        mutated_this_loop = true;
                    }
                }
            }

            // Go through the remaining empty spots, for each one, try to find a
            // character that would fit
            for r in 0..4 {
                for c in 0..4 {
                    if !empty[r][c] {
                        continue;
                    }

                    let candidate_chars = rows[r]
                        .iter()
                        .chain(columns[c].iter())
                        .copied()
                        .filter(|&x| x != '?')
                        .filter(|x| !used_chars.contains(x))
                        .filter(|x| {
                            let row_contains = rows[r].contains(x);
                            let col_contains = columns[c].contains(x);

                            // exactly one of the row or the column contains this char
                            row_contains ^ col_contains
                        })
                        .collect::<Vec<_>>();

                    if candidate_chars.len() != 1 {
                        continue;
                    }

                    let candidate_char = candidate_chars[0];
                    let row_contains = rows[r].contains(&candidate_char);
                    let col_contains = columns[c].contains(&candidate_char);

                    if row_contains {
                        let qmark_pos = match columns[c].iter().position(|&x| x == '?') {
                            Some(pos) => pos,
                            None => continue,
                        };
                        columns[c][qmark_pos] = candidate_char;

                        qmark_replacements.push((Specifier::Col(c, qmark_pos), candidate_char))
                    } else if col_contains {
                        let qmark_pos = match rows[r].iter().position(|&x| x == '?') {
                            Some(pos) => pos,
                            None => continue,
                        };
                        rows[r][qmark_pos] = candidate_char;

                        qmark_replacements.push((Specifier::Row(r, qmark_pos), candidate_char))
                    } else {
                        unreachable!()
                    }

                    word.push(((r, c), candidate_char));
                    used_chars.push(candidate_char);
                    empty[r][c] = false;

                    mutated_this_loop = true;
                }
            }

            if !mutated_this_loop {
                break;
            }
        }

        let mut mutated_overall = false;

        // Make any qmark replacements we found on the actual grid
        for (specifier, char) in qmark_replacements {
            let (rx, ry) = match specifier {
                Specifier::Row(r, i) => (x * 6 + i + if i >= 2 { 4 } else { 0 }, y * 6 + r + 2),
                Specifier::Col(c, i) => (x * 6 + c + 2, y * 6 + i + if i >= 2 { 4 } else { 0 }),
            };
            self.grid[ry][rx] = char;

            mutated_overall = true;
        }

        if word.len() == 16 {
            word.sort();

            // Put the word onto the grid
            for ((r, c), char) in word.iter() {
                let (rx, ry) = (x * 6 + c + 2, y * 6 + r + 2);
                self.grid[ry][rx] = *char;
            }

            (
                Some(word.iter().map(|(_, c)| *c).collect()),
                mutated_overall,
            )
        } else {
            (None, mutated_overall)
        }
    }

    fn dbg_print(&self) {
        for row in self.grid.iter() {
            for char in row {
                print!("{char}");
            }
            println!();
        }
    }
}

fn part3(input: &str) -> u32 {
    let big_grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let width_blocks = big_grid[0].len() / 6;
    let height_blocks = big_grid.len() / 6;

    let mut wall = Wall {
        grid: big_grid,
        width_blocks,
        height_blocks,
    };

    let mut solved = vec![vec![false; wall.width_blocks]; wall.height_blocks];
    let mut total = 0;

    loop {
        let mut mutated_this_loop = false;

        for y in 0..wall.height_blocks {
            for x in 0..wall.width_blocks {
                if solved[y][x] {
                    continue;
                }

                let (word, mutated) = wall.try_solve_at(x, y);

                if let Some(word) = word {
                    solved[y][x] = true;
                    mutated_this_loop = true;
                    total += runic_word_power(&word);
                }

                if mutated {
                    mutated_this_loop = true;
                }
            }
        }

        if !mutated_this_loop {
            break;
        }
    }

    wall.dbg_print();

    total
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
