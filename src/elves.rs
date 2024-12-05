use std::fs::read_to_string;
use grid::Grid;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn read_grid(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x: &str| x.chars().collect())
        .collect() 
}

