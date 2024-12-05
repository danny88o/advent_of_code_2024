mod days;
pub mod solution;
pub mod elves;

use std::env;

use days::day04::part_2 as part_2;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS", "-Awarnings");
    part_2();
}
