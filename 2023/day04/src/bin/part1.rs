use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}

fn parse_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect()
}
fn sol(input: &str) -> String {
    let mut total = 0;
    let lines:Vec<_> = input.split('\n').collect();
    for l in lines {
        let ind = l.find(":").unwrap();
        let numbers = &l[ind+1..];
        let parts :Vec<_> = numbers.split('|').collect();
        let winners = parse_numbers(parts.first().unwrap());
        let winners_set: HashSet<i32> = winners.into_iter().collect();
        let ticket = parse_numbers(parts.last().unwrap());
        let mut points = 0;
        for t in ticket {
            if (winners_set.contains(&t)){
                if (points==0){
                    points = 1;
                }else{
                    points*=2;
                }
            }
        }
        total+=points;
    }
    String::from(total.to_string())
}