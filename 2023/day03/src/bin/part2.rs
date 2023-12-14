use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}
fn check_char(c: char)-> bool{
    c!='.' && !c.is_digit(10)
}

struct GearNum{
    x:i32,
    y:i32,
    num:i32
}
fn sol(input: &str) -> String {
    let mut total = 0;
    let lines:Vec<_> = input.split('\n').collect();
    let mut lines_new: Vec<String> = Vec::new();

    let mut tmp_str:String = String::from("");
    for i in 0..lines.first().unwrap().len()+2 {
        tmp_str+=".";
    }
    lines_new.push(tmp_str.clone());

    let mut gears: Vec<GearNum> = Vec::new();

    for (ind, line) in lines.iter().enumerate() {
        let formatted_line = format!(".{}.", line);
        lines_new.push(formatted_line);
    }
    lines_new.push(tmp_str);
    let rows = lines_new.len();

    for (row,line) in lines_new.iter().enumerate() {
        if (row==rows-1){
            break;
        }
        let cols = line.len() as i32;
        let mut num:i32 = 0;
        let mut start = false;
        let mut left:i32 = 0;
        let mut right:i32 = 0;

        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if !start {
                    start = true;
                    num = c.to_digit(10).unwrap() as i32;
                    left = col as i32;
                    right = col as i32;
                } else {
                    right = col as i32;
                    num = num * 10 + c.to_digit(10).unwrap() as i32;
                }
            }
            else {
                if num!=0 {
                    if (num == 179) {
                        let a=3;
                        num+=10;
                        num-=10;
                    }
                    let mut flag = false;
                    let l = cmp::max(left-1, 0);
                    let r = cmp::min(right+1, cols);
                    for x in row-1..=row+1{
                        for y in l..=r{
                            if check_char(lines_new[x].as_bytes()[y as usize] as char) {
                                if (lines_new[x].as_bytes()[y as usize] as char == '*'){
                                    gears.push(GearNum{x:x as i32,y:y as i32, num:num});
                                }
                                flag = true;
                                break;
                            }
                        }
                    }
                    num = 0;
                    start = false;
                }
            }
        }
    }
    total = 0;
    let mut groups: HashMap<i32, Vec<&GearNum>> = HashMap::new();

    for gear_num in &gears {
        let key = gear_num.x * 100000 + gear_num.y;
        groups.entry(key).or_insert(vec![]).push(gear_num);
    }

    let result2: Vec<&Vec<&GearNum>> = groups.values().filter(|group| group.len() > 1).collect();

    let mut result =0;
    for group in result2 {
        let mut  prod = 1;
        for el in group{
            prod*=el.num;
        }
        result+=prod;
    }
    String::from(result.to_string())
}