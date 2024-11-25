use utils::read_input_files;

fn potions_needed(c: u8) -> Option<i64> {
    match c {
        b'A' => Some(0),
        b'B' => Some(1),
        b'C' => Some(3),
        b'D' => Some(5),
        _ => None,
    }
}

fn part1(s: &str) -> i64 {
    s.trim().bytes().filter_map(potions_needed).sum()
}

fn part2(s: &str) -> i64 {
    let potions_needed: Vec<_> = s.trim().bytes().map(potions_needed).collect();

    potions_needed
        .chunks_exact(2)
        .map(|chunk| {
            let total = chunk.iter().map(|x| x.unwrap_or(0)).sum::<i64>();
            if chunk.iter().all(|x| x.is_some()) {
                total + 2
            } else {
                total
            }
        })
        .sum()
}

fn part3(s: &str) -> i64 {
    let potions_needed: Vec<_> = s.trim().bytes().map(potions_needed).collect();

    potions_needed
        .chunks_exact(3)
        .map(|chunk| {
            let total = chunk.iter().map(|x| x.unwrap_or(0)).sum::<i64>();
            let monsters = chunk.iter().map(|x| i32::from(x.is_some())).sum();

            match monsters {
                3 => total + 6,
                2 => total + 2,
                _ => total,
            }
        })
        .sum()
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
