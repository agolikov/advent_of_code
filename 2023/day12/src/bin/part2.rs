use std::hash::Hash;
use std::os::macos::raw::stat;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}

fn calculate_count(p0: &str, nums: &Vec<usize>, pos:usize, chars: &mut Vec<char>) -> i32
{
    let mut total = 0;
    if (chars.len() == p0.len()) {
        let mut curLeng = 0;
        let mut state = false;
        let mut seq : Vec<usize> = Vec::new();
        for i in 0..chars.len() {
            if (chars[i]=='.')
            {
                if (!state && curLeng > 0) {
                    seq.push(curLeng);
                    if (seq.len() > nums.len()) {
                        return 0;
                    }
                    curLeng = 0;
                }
                state = true;
            }else
            {
                curLeng +=1;
                state = false;
            }
        }
        if (curLeng>0) {seq.push(curLeng);}
        if (seq.len() == nums.len()){
            for i in 0..seq.len(){
                if seq[i]!=nums[i]{
                    return 0;
                }
            }
            return 1;
        }

    } else {
        if (p0.as_bytes()[pos] as char == '.' || p0.as_bytes()[pos] as char =='?') {
            chars.push('.');
            total += calculate_count(p0, nums, pos+1, chars);
            chars.pop();
        }
        if (p0.as_bytes()[pos] as char == '#' || p0.as_bytes()[pos] as char =='?') {
            chars.push('#');
            total += calculate_count(p0, nums, pos+1, chars);
            chars.pop();
        }
    }
    return total;
}

fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let mut total:i32 = 0;
    for (i, s) in lines.iter().enumerate() {
        let mut parts:Vec<&str> = s.split(' ').collect();
        let line = parts[0].repeat(5);
        let numbers = parts[1].repeat(5);
        let nums: Vec<usize> = numbers.split(',').map(|s| s.parse().expect("Failed to parse number")).collect();
        let add = calculate_count(&line, &nums, 0,  &mut Vec::new() );
        println!("{} {}", i+1, add);
        total+=add;
    }

    String::from(total.to_string())
}