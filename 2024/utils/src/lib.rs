pub mod cycle_detection;
pub mod grid;

use std::borrow::Cow;

pub fn read_input_files_internal(defaults: [&'static str; 3]) -> [Cow<'static, str>; 3] {
    let args: Vec<_> = std::env::args().collect();

    let mut filenames = [None; 3];

    for window in args.windows(2) {
        let [a, b] = window else { unreachable!() };
        if a == "-p1" || a == "--part1" {
            filenames[0] = Some(b);
        }

        if a == "-p2" || a == "--part2" {
            filenames[1] = Some(b);
        }

        if a == "-p3" || a == "--part3" {
            filenames[2] = Some(b);
        }
    }

    filenames
        .iter()
        .enumerate()
        .map(|(i, filename)| match filename {
            Some(filename) => Cow::from(std::fs::read_to_string(filename).unwrap()),
            None => Cow::from(defaults[i]),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[macro_export]
macro_rules! read_input_files {
    () => {{
        $crate::read_input_files_internal([
            include_str!("../part1.txt"),
            include_str!("../part2.txt"),
            include_str!("../part3.txt"),
        ])
    }};
}
