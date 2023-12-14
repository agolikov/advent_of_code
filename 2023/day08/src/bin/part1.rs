use std::{cmp, fmt};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ptr::hash;

fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}

fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let mv = &lines[0];
    let n = mv.len();
    let mut sh: [i32; 1000] = [0; 1000];
    for i in 0..mv.len(){
        if mv.chars().nth(i).unwrap() =='R' {
            sh[i]=1;
        }
    }
    let mut map :HashMap<String, (String, String)> = HashMap::new();
    let mut st =String::from("AAA");
    for l in lines.iter().skip(2) {
        let a = &l[0..3];
        let b = &l[7..10];
        let c = &l[12..15];
        map.insert(String::from(a),(String::from(b),String::from(c)));
    }
    let mut count = 0;
    let mut id = 0;
    while(st!="ZZZ"){
        let (left, right) = map.get(&st).unwrap();
        if (sh[id]==0)
        {
            st = String::from(left);

        }else{
          st = String::from(right);
        }
        id+=1;
        if (id ==n) {
            id = 0;
        }
        count+=1;
    }
    String::from(count.to_string())
}