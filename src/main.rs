
#[macro_use]
extern crate codegen;

use rand::{thread_rng, Rng};

mod design_pattern;
mod data_structure;
mod gaming;
mod core;
mod alg;

use codegen::simple;
macro_rules! recognise_tree {
    (larch) => { println!("#1, the Larch.") };
    (redwood) => { println!("#2, the Mighty Redwood.") };
    (fir) => { println!("#3, the Fir.") };
    (chestnut) => { println!("#4, the Horse Chestnut.") };
    (pine) => { println!("#5, the Scots Pine.") };
    ($($other:tt)*) => { println!("I don't know; some kind of birch maybe?") };
}
fn main() {
    simple!(1+1);
    recognise_tree!(larch);
}
fn gaming(){
    gaming::snake::run();
}
