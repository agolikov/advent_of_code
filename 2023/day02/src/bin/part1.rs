fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}

fn sol(input: &str) -> String {
    let lines = input.split('\n');
    let mut total = 0;
    let mut id = 0;
    'outer: for l in lines {
        id+=1;
        //Game 30: 9 red, 18 green, 6 blue; 1 green, 3 blue, 5 red; 6 blue, 12 green, 3 red
        let ind = l.find(":").unwrap();
        let games= (&l[(ind+1)..]).split(';');
        for game in games {
            let parts = game.split(',');
            for part in parts {
                let num = part.clone().replace("green","").replace("red","").replace("blue","").trim().parse::<i32>().unwrap();
                if part.find("green").is_some() && num > 13 { continue 'outer }
                if part.find("red").is_some() && num > 12 { continue 'outer }
                if part.find("blue").is_some() && num > 14 { continue 'outer }
            }
        }
        total+=id;
    }
    String::from(total.to_string())
}
