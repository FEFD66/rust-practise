#![feature(option_result_contains)]

use rand::{thread_rng, Rng};

mod design_pattern;
mod data_structure;
mod gaming;
mod core;
mod alg;

fn main() {
    println!("Hello, world!");
    let mut rng =thread_rng();
    for _ in 0..2000{
        println!("{}",rng.gen_range(0..100))
    }
    gaming::snake::run();
}
