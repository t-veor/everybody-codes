use utils::{bisect::exponential_search, read_input_files};

fn blocks_for_pyramid_with_layers(layers: i64) -> i64 {
    // Sum (2n + 1) from 0 to x = (x + 1)^2
    (layers + 1).pow(2)
}

fn max_completed_pyramid_layers(blocks: i64) -> i64 {
    // This is the inverse of f(x) = x(x + 2).
    let x = blocks as f64;
    x.sqrt().floor() as i64 - 1
}

fn part1(input: &str) -> i64 {
    let blocks = input.trim().parse().unwrap();

    let layers = max_completed_pyramid_layers(blocks);
    let current_layer_blocks = blocks_for_pyramid_with_layers(layers);

    if blocks == current_layer_blocks {
        0
    } else {
        let blocks_needed = blocks_for_pyramid_with_layers(layers + 1) - blocks;
        let base_width = 2 * layers + 1 + (blocks - current_layer_blocks).min(2);

        blocks_needed * base_width
    }
}

fn part2(input: &str) -> i64 {
    let multiplier: i64 = input.trim().parse().unwrap();
    let modulus = 1111;
    let blocks = 20240000;

    let mut blocks_needed = 0;
    let mut thickness = 1;
    let mut width = 1;
    loop {
        blocks_needed += thickness * width;

        if blocks_needed >= blocks {
            break;
        }

        width += 2;
        thickness = (thickness * multiplier) % modulus;
    }

    width * (blocks_needed - blocks)
}

#[derive(Debug)]
struct Pyramid {
    priests: i64,
    acolytes: i64,
    solid_blocks: i64,
    layer_thicknesses: Vec<i64>,
}

impl Pyramid {
    fn width(&self) -> i64 {
        (self.layer_thicknesses.len() as i64 * 2 - 1).max(0)
    }

    fn apply_next_layer(&mut self) {
        let thickness = if let Some(prev_thickness) = self.layer_thicknesses.last() {
            (prev_thickness * self.priests) % self.acolytes + self.acolytes
        } else {
            1
        };
        self.layer_thicknesses.push(thickness);

        self.solid_blocks += self.width() * thickness;
    }

    fn removable_blocks(&self) -> i64 {
        let mut removed_blocks = 0;
        let mut prev_column_height;
        let mut column_height = 0;

        let width = self.width();

        for i in (0..self.layer_thicknesses.len()).rev() {
            prev_column_height = column_height;
            column_height += self.layer_thicknesses[i];

            let must_untouched_blocks = if i == self.layer_thicknesses.len() - 1 {
                column_height
            } else {
                column_height - prev_column_height + 1
            };

            let should_remove_blocks = (width * self.priests * column_height) % self.acolytes;
            let removed_this_column =
                (column_height - must_untouched_blocks).min(should_remove_blocks);

            if i == 0 {
                // Central column
                removed_blocks += removed_this_column
            } else {
                // Two of this column on either side of the centre
                removed_blocks += 2 * removed_this_column;
            }
        }

        removed_blocks
    }

    fn total_blocks(&self) -> i64 {
        self.solid_blocks - self.removable_blocks()
    }

    fn with_layers(priests: i64, acolytes: i64, layers: u64) -> Self {
        let mut pyramid = Self {
            priests,
            acolytes,
            solid_blocks: 0,
            layer_thicknesses: Vec::with_capacity(layers as usize),
        };

        for _ in 0..layers {
            pyramid.apply_next_layer();
        }

        pyramid
    }
}

fn part3(input: &str) -> i64 {
    let priests = input.trim().parse().unwrap();
    let acolytes = 10;
    let blocks = 202400000;

    let target_layers = exponential_search(0, |layers| {
        Pyramid::with_layers(priests, acolytes, layers).total_blocks() < blocks
    });

    let pyramid = Pyramid::with_layers(priests, acolytes, target_layers);

    pyramid.total_blocks() - blocks
}

fn main() {
    let [p1, p2, p3] = read_input_files!();

    println!("{}", part1(&p1));
    println!("{}", part2(&p2));
    println!("{}", part3(&p3));
}
