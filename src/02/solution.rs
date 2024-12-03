


//use crate::elves::read_lines as read_lines;

//pub mod read_file;


use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn is_safe(nums: Vec<i32>, min_change: i32, max_change: i32) -> bool {
    let increasing = (nums[1]-nums[0]).is_positive();
    for i in 1..nums.len() {
        let curr = nums[i-1];
        let next = nums[i];

        let diff = next - curr;

        if (diff.abs() < min_change) || (diff.abs() > max_change) {return false};

        let new_increasing = diff.is_positive();
        if increasing ^ diff.is_positive() {return false};
    }
    return true;
}

fn str_to_ints(s: String) -> Vec<i32> {
    let chrs: Vec<&str> = s.split_whitespace().collect();

    let mut nums = Vec::new();
    for c in chrs.iter() {
        let c2 = c.parse::<i32>().unwrap();
        nums.push(c2);
    }
    return nums;
}

fn part_1() {
    let lines = read_lines("input");

    let mut count = 0;
    for line in lines {
        let nums = str_to_ints(line);
        if is_safe(nums, 1 , 3) {count = count + 1}
    }
    println!("{}", count)
}



fn part_2() {
    let lines = read_lines("input");

    let mut count = 0;
    for line in lines {
        let nums = str_to_ints(line);
        if is_safe(nums.clone(), 1 , 3) {
            count = count + 1
        }
        else {
            for i in 0..nums.len() {
                let mut nums2 = nums.clone();
                nums2.remove(i);
                if is_safe(nums2, 1 , 3) {count = count + 1; break}
            }
        }
    }
    println!("{}", count)
}

fn main() {
    part_1();
    part_2()
}