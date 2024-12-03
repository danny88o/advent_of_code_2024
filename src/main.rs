mod days;
pub mod solution;
pub mod elves;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    days::day03::part_2()
}
