#![feature(option_result_contains)]

use rand::{thread_rng, Rng};

mod design_pattern;
mod collection;
mod gaming;
mod core;

fn main() {
    println!("Hello, world!");
    let mut rng =thread_rng();
    for _ in 0..2000{
        println!("{}",rng.gen_range(0..100))
    }
    gaming::snake::run();
}
