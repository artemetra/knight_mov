use std::time::Instant;

use knight_mov::{get_min_area, get_min_displacement};

const A: i32 = 13;
const B: i32 = 10;
const STEPS: usize = 4;

fn main() {
    let start_time = Instant::now();
    let minimum = get_min_displacement(A, B, STEPS);
    let duration = start_time.elapsed();
    println!();
    println!(
        "Calculated minimum for a={}, b={}, steps={} in {:?}.",
        A, B, STEPS, duration
    );
    println!(
        "{:?}, {:?}",
        minimum,
        (minimum.displacement.norm_sqr() as f32).sqrt()
    );
    let min_area = get_min_area(&minimum);
    println!("{:?}", min_area);
}
