use crate::elves::read_lines as read_lines;

pub fn part_1() {
    let lines: Vec<String> = read_lines("input/day07");
    
    let mut total = 0;
    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();

        let test_total: i64 = split[0].parse().unwrap();
        let mut nums: Vec<i64> = split[1].split(" ").into_iter().map(|x: &str| x.parse().unwrap()).collect();
        nums.reverse();
        let combinations: Vec<i64> = combinations(nums, test_total);
        //println!("{:?}", combinations);
        if combinations.contains(&test_total) {
            total += test_total;
        }
    }
    println!("{}", total)
}

fn concat(x:&i64, y:&i64) -> i64 {
    (x.to_string() + &y.to_string()).parse().unwrap()
}

fn combinations(nums: Vec<i64>, target: i64) -> Vec<i64> {
    if let Some((head, tail)) = nums.split_first() {
        if tail.is_empty() {
            return [*head].to_vec();
        }
        else {
            let mut all_combinations = Vec::new();
            for comb in combinations(tail.to_vec(), target) {
                let plus = head + comb;
                if plus <= target {
                    all_combinations.push(plus);
                }
                let mult = head * comb;
                if mult <= target {
                    all_combinations.push(mult);
                }
                let conc = concat(&comb, head);
                if conc <= target {
                    //println!("{} {} {}", *head, conc, comb);

                    all_combinations.push(conc);
                }
            }
            return all_combinations;
        }
    }
    else {
        panic!("Empty combinations");
    }
}

