use std::hash::Hash;
use std::os::macos::raw::stat;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}

fn calculate_count(line: &str, nums: &Vec<usize>, pos:usize, id: usize, chars:&mut Vec<char>) -> i32
{
    let mut total = 0;
    if (id == nums.len()) {
        if (chars.len() <= line.len()+1) {
            let x = &line[chars.len()..];
            if (x.contains('#')){
                return 0;
            }
            // println!("{:?}", chars);
            return 1;
        }

    } else {
        if (pos < line.len()) {
            if (line.as_bytes()[pos] as char == '.' || line.as_bytes()[pos] as char == '?') {
                chars.push('.');
                total += calculate_count(line, nums, pos + 1, id, chars);
                chars.pop();
            }
            if (pos > 0 && chars[pos - 1] != '.') {
                return total;
            }
            if (nums[id] + pos > line.len()) {
                return total;
            }

            for i in 0..nums[id] {
                let c = line.as_bytes()[pos + i] as char;
                if c == '.' {
                    return total;
                }
            }
            if !(line.as_bytes()[pos + nums[id]] as char != '#') {
                return total;
            }
            let add = "#".repeat(nums[id]) + ".";
            for c in add.chars() {
                chars.push(c);
            }
            total += calculate_count(line, nums, pos + nums[id] + 1, id + 1, chars);
            for c in add.chars() {
                chars.pop();
            }
        }
    }
    return total;
}

fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let mut total:i32 = 0;
    for (i, s) in lines.iter().enumerate() {
        let mut parts:Vec<&str> = s.split(' ').collect();
        let line = parts[0].to_owned().repeat(5) + ".";
        let numbers = (parts[1].to_owned() + ",").repeat(5);

        let nums: Vec<usize> = numbers[..numbers.len()-1].split( ',').map(|s| s.parse().expect("Failed to parse number")).collect();
        let add = calculate_count(&line, &nums, 0, 0, &mut Vec::new());
        println!("{} {}", i+1, add);
        total+=add;
    }

    String::from(total.to_string())
}