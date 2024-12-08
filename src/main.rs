#![allow(dead_code)]
mod days;
pub mod solution;
pub mod elves;

use std::env;

use days::day07::part_1 as part_1;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS", "-Awarnings");
    part_1();
}
