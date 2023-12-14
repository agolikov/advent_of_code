use std::env;
use std::fs;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}

struct Number {
    position: usize,
    num:u32
}

fn TryAddNumber(ln:&str, numStr:&str, numV:u32, nums: &mut Vec<Number>) {
    let index:Vec<_> = ln.match_indices(numStr).collect();
    for (_i,str) in index {
        let x = Number {position:_i, num:numV };
        nums.push(x)
    }
}
fn sol(input: &str) -> String {

    let lines = input.split('\n');
    let mut total = 0;
    for l in lines {
        let mut vec = Vec::new();
        TryAddNumber(l, "one", 1, &mut vec);
        TryAddNumber(l,"two",2, &mut vec);
        TryAddNumber(l,"three",3, &mut vec);
        TryAddNumber(l,"four",4, &mut vec);
        TryAddNumber(l,"five",5, &mut vec);
        TryAddNumber(l,"six",6, &mut vec);
        TryAddNumber(l,"seven",7, &mut vec);
        TryAddNumber(l,"eight",8, &mut vec);
        TryAddNumber(l,"nine",9, &mut vec);
        for (_i, c) in l.chars().enumerate() {
            if c.is_numeric() {
                let x  = c.to_digit(10).unwrap();
                vec.push(Number{position:_i, num:x});
            }
        }
        vec.sort_by(|x,y|{x.position.cmp(&y.position)});
        total +=vec.first().unwrap().num*10 + vec.last().unwrap().num;
    }
    String::from(total.to_string())
}
