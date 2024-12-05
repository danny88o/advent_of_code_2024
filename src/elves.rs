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

pub fn read_split(filename: &str, split_on: &str) -> (Vec<String>, Vec<String>) {
    let mut file_sec_1: Vec<String> = Vec::new();
    let mut file_sec_2: Vec<String> = Vec::new();
    let mut sec_1 = true;
    for line in read_to_string(filename).unwrap().lines() {
        if line == split_on {
            sec_1 = false;
            continue;
        }
        if sec_1 {
            file_sec_1.push(line.to_string());
        }
        else {
            file_sec_2.push(line.to_string());
        }
    }
    return (file_sec_1, file_sec_2);
}

pub fn get_middle_val<T>(v: &Vec<T>) -> T
where T: Clone {
    let mid = v.len() / 2;
    return v[mid].clone();
}