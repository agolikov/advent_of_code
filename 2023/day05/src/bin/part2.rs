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
fn sol(input: &str) -> String {
    let lines:Vec<_> = input.split('\n').collect();
    let seeds = &lines.first().unwrap()[6..];
    let seedNumbers = parse_numbers(seeds);
    let mut row = 3;
    let mut maps: Vec<Vec<(i64, i64,i64)>> = Vec::new();
    let mut cur: Vec<(i64, i64,i64)> = Vec::new();
    while(row < lines.len())
    {
        if (lines[row].contains("map")) {
            maps.push(cur);
            cur = Vec::new();
        }else{
            if (!lines[row].is_empty()){
                let nums=parse_numbers(lines[row]);
                cur.push((nums[0],nums[1],nums[2]));
            }
        }
        row+=1;
    }
    maps.push(cur);
    let mut min = -1;
    for i in (0..seedNumbers.len()).step_by(2) {
        for c in seedNumbers[i]..seedNumbers[i]+seedNumbers[i+1] {
            let mut index = c;
            for map in &maps {
                for el in map {
                    let destination: i64 = el.0;
                    let source: i64 = el.1;
                    let shift: i64 = el.2;
                    if (source <= index && source + shift >= index) {
                        index += (destination - source);
                        break;
                    }
                }
            }
            if min == -1 || index < min {
                min = index;
                println!("found in i={}",i);
            }
        }
    }
    String::from(min.to_string())
}