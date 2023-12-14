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
    let time = *&lines[0][5..].replace(" ","").parse::<i128>().unwrap();
    let distance = *&lines[1][9..].replace(" ","").parse::<i128>().unwrap();
    let mut count = 0;
    for t in 0..time {
        let dist = (time-t)*(t);
        if (dist > distance) {
            count +=1;
        }
    }
    String::from(count.to_string())
}