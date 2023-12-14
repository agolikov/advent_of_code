use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;

fn main() {
    let input = include_str!("input2.txt");
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
    let lines:Vec<_> = input.split('\n').collect();
    let mut cards = HashMap::new();
    let mut cardId:usize = 0;
    for i in 1..=lines.len(){
        cards.insert(i,1);
    }

    for l in lines {
        cardId+=1;
        let mul: i32 = cards.get(&cardId).unwrap().clone();
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
                    points+=1;
                }
            }
        }
        for i in 1..=points {
            let id = cardId + i;
            match cards.get(&id) {
                Some(v) => {
                    cards.insert(id,v + mul);
                },
                None => {
                    cards.insert(id, mul);
                }
            }
        }
    }
    let total: i32 = cards.values().cloned().sum();
    String::from(total.to_string())
}