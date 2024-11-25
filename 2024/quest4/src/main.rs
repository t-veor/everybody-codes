use utils::read_input_files;

fn minimum_strikes_part1(nail_heights: &[i64]) -> i64 {
    let min = nail_heights.iter().min().copied().unwrap_or(0);
    nail_heights.iter().sum::<i64>() - min * nail_heights.len() as i64
}

fn part1(input: &str) -> i64 {
    let nail_heights: Vec<_> = input.trim().lines().map(|i| i.parse().unwrap()).collect();

    minimum_strikes_part1(&nail_heights)
}

fn part2(input: &str) -> i64 {
    part1(input)
}

fn median(values: &[i64]) -> i64 {
    let mut sorted = values.to_vec();
    sorted.sort();

    let middle_idx = sorted.len() / 2;
    sorted[middle_idx]
}

fn minimum_strikes_part3(nail_heights: &[i64]) -> i64 {
    let med = median(nail_heights);
    nail_heights.iter().map(|x| x.abs_diff(med) as i64).sum()
}

fn part3(input: &str) -> i64 {
    let nail_heights: Vec<_> = input.trim().lines().map(|i| i.parse().unwrap()).collect();

    minimum_strikes_part3(&nail_heights)
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
