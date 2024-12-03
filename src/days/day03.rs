use regex::Regex;
use std::fs::read_to_string;


fn get_muls(s:&str) -> Vec<&str> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    return re.find_iter(s).map(|m| m.as_str()).collect();
}

fn intrpret_mul(mul: &str) -> i32 {
    let re: Regex = Regex::new(r"(\d+)").unwrap();
    let cap: Vec<&str> = re.captures_iter(mul).map(|caps| {
        let (_, [cap]) = caps.extract();
        cap
    }).collect();
    let first: i32  = cap[0].parse().unwrap();
    let second: i32 = cap[1].parse().unwrap();

    return first * second;
}

pub fn part_1() {
    let file: String = read_to_string("practice/day03").unwrap();
    let mults: Vec<&str> = get_muls(&file);
    let total = mults.iter().fold(0, |acc, mul| acc + intrpret_mul(mul));
    println!("{}", total)

}


fn get_all(s:&str) -> Vec<&str> {
    let re = Regex::new(r"(?:mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();
    return re.find_iter(s).map(|m| m.as_str()).collect();
} 

pub fn part_2() {
    let file: String = read_to_string("input/day03").unwrap();

    let commands: Vec<&str> = get_all(&file);
    let mut parse = true;
    let mut total = 0;
    for command in commands {
        match command {
            "don't()" => parse = false,
            "do()" => parse = true,
            _ => if parse {total += intrpret_mul(command)}
        }
    }
    println!("{}", total)
}
