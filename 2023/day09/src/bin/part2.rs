use std::{cmp, fmt};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ptr::hash;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect()
}

fn get_prev(nums: Vec<i64>) -> i64 {
    let mut diff:Vec<i64> = Vec::new();
    let mut zero = true;
    for x in 1..nums.len() {
        diff.push((nums[x]-nums[x-1]));
        if (nums[x]-nums[x-1]!=0) {
            zero =false;
        }
    }
    if zero {
        return nums[0];
    } else
    {
        return nums[0] - get_prev(diff);
    }
}
fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let mut total:i64 = 0;
    for l in lines {
        let arr = parse_numbers(l);
        let cur = get_prev(arr);
        println!("{}, {}",cur, l);
        total +=cur;
    }
    String::from(total.to_string())
}