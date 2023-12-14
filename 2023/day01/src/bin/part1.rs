use std::env;
use std::fs;

fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}

fn sol(input: &str) -> String {
    let lines = input.split('\n');
    let mut total = 0;
    for l in lines {
        let mut first: bool = false;
        let mut right = 0;
        let mut left = 0;
        //let out :Vec<_>= l.chars().map(move |x| x.to_digit(10)).collect();

        for (_i, c) in l.chars().enumerate() {
            if c.is_numeric() {
                if !first {
                    right = c.to_digit(10).unwrap();
                    first = true;
                }
                left = c.to_digit(10).unwrap();
            }
        }
        total += right*10 + left;
    }
    String::from(total.to_string())
}
