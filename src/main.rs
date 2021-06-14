use crate::demo::features::feature_test;

mod design_pattern;
mod collection;
mod gaming;
mod demo;

fn main() {
    println!("Hello, world!");
    gaming::snake::run();
}
