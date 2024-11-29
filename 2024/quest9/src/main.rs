use std::u32;

use utils::read_input_files;

struct ChangeMaker {
    coins: Vec<usize>,
    memo: Vec<u32>,
}

impl ChangeMaker {
    fn new(coins: &[usize]) -> Self {
        Self {
            coins: coins.to_vec(),
            memo: vec![0],
        }
    }

    fn calculate_memo(&mut self, target: usize) -> u32 {
        for n in self.memo.len()..=target {
            let mut min_coins = u32::MAX;

            for &coin in self.coins.iter() {
                if n >= coin {
                    min_coins = min_coins.min(self.memo[n - coin] + 1);
                }
            }

            self.memo.push(min_coins);
        }

        self.memo[target]
    }
}

fn part1(input: &str) -> u32 {
    let targets = input.trim().lines().map(|x| x.parse().unwrap());
    let mut change_maker = ChangeMaker::new(&[1, 3, 5, 10]);

    let mut total = 0;
    for target in targets {
        total += change_maker.calculate_memo(target);
    }

    total
}

fn part2(input: &str) -> u32 {
    let targets = input.trim().lines().map(|x| x.parse().unwrap());
    let mut change_maker = ChangeMaker::new(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30]);

    let mut total = 0;
    for target in targets {
        total += change_maker.calculate_memo(target);
    }

    total
}

fn part3(input: &str) -> u32 {
    let targets = input.trim().lines().map(|x| x.parse::<usize>().unwrap());
    let mut change_maker = ChangeMaker::new(&[
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ]);

    let mut total = 0;
    for target in targets {
        if target == 0 {
            continue;
        }

        let mut best_stamps_for_this_target = u32::MAX;

        let mut left_beetles = ((target + 1) / 2).saturating_sub(50);
        loop {
            let right_beetles = target.saturating_sub(left_beetles);
            if right_beetles < left_beetles {
                break;
            }

            let left_stamps = change_maker.calculate_memo(left_beetles);
            let right_stamps = change_maker.calculate_memo(right_beetles);

            let stamps = left_stamps + right_stamps;
            best_stamps_for_this_target = best_stamps_for_this_target.min(stamps);

            left_beetles += 1;
        }

        total += best_stamps_for_this_target;
    }

    total
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
