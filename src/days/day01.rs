use crate::elves::read_lines as read_lines;
use std::collections::HashMap;

fn part_1() -> i32 {
    let lines = read_lines("input");

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        let x: i32 = nums[0].parse().unwrap();
        let y: i32 = nums[1].parse().unwrap();
        xs.push(x);
        ys.push(y);
    }
    xs.sort();
    ys.sort();

    assert_eq!(xs.len(), ys.len());

    return xs.iter().zip(ys.iter()).fold(
        0, |acc, (x, y)| acc + (x - y).abs()
    );
}


fn part_2() -> i32 {
    let lines = read_lines("practice");

    let mut xs = Vec::new();
    let mut ys: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let nums: Vec<&str> = line.split_whitespace().collect();
        let x: i32 = nums[0].parse().unwrap();
        let y: i32 = nums[1].parse().unwrap();
        xs.push(x);
        *ys.entry(y).or_insert(0) += 1;
    }

    return xs.iter().fold(
        0, |acc, x| acc + x*ys.get(x).unwrap_or(&0)
    ); 
}

fn main() {
    let ans = part_2();
    println!("{}", ans);
}