use crate::elves::read_split as read_split;
use crate::elves::get_middle_val as get_middle_val;


type Page = i32;

#[derive(Clone)]
struct Rule (Page, Page);

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 
    }
}
impl Eq for Rule {}

#[derive(Clone)]
struct OrdPage {
    num: Page,
    rules: Vec<Rule>
}

impl PartialEq for OrdPage {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Eq for OrdPage {}

impl Ord for OrdPage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.num == other.num {
            return std::cmp::Ordering::Equal
        }
        else if self.rules.contains(&Rule(self.num, other.num)) {
            return std::cmp::Ordering::Less
        }
        else {
            return std::cmp::Ordering::Greater
        }
    }
}

impl PartialOrd for OrdPage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}



pub fn solution() {
    let (rules_str, order_str) = read_split("input/day05", "");
    let rules: Vec<Rule> = to_rules(rules_str);
    let ordpages: Vec<Vec<OrdPage>> = order_str
        .iter()
        .map(|pages| pages
            .split(",")
            .map(|page| {let x = page.parse::<Page>().unwrap(); OrdPage{num: x, rules: rules.clone()}})
            .collect())
        .collect();
    
    
    let mut total_1 = 0;
    let mut total_2 = 0;
    for pages in ordpages.iter() {
        if is_sorted(pages) {
            total_1 += get_middle_val(pages).num;
        }
        else {
            let mut pages2 = pages.clone();
            pages2.sort();
            total_2 += get_middle_val(&pages2).num;
        }   
    }
    println!("{}", total_1);
    println!("{}", total_2);

}

fn to_rules(vec_str: Vec<String>) -> Vec<Rule> {
    vec_str.iter().map(|x| {
        let nums: Vec<Page> = x
            .split("|")
            .map(|x| x.parse::<Page>().unwrap())
            .collect();
        Rule(nums[0], nums[1])
    }).collect()
}
