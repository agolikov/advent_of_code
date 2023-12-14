fn main() {
    let input = include_str!("input1.txt");
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
    let times = &lines[0][5..];
    let times = parse_numbers(times);
    let distances = &lines[1][9..];
    let distances = parse_numbers(distances);
    let mut product = 1;
    for i in 0..times.len() {
        let mut count = 0;
        for t in 0..times[i] {
            let dist = (times[i]-t)*(t);
            if (dist > distances[i]) {
                count +=1;
            }
        }
       // println!("{}", count);
        product *=count;
    }
    String::from(product.to_string())
}