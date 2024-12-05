mod days;
pub mod solution;
pub mod elves;

use std::env;

use days::day05::solution as solution;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUSTFLAGS", "-Awarnings");
    solution();
}
