use std::collections::{HashMap};
use std::hash::Hash;
use lazy_static::lazy_static;
use rand::random;

fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}


fn get_char(x:i32, y:i32, map:&Vec<&str>) -> char{
    map.get(x as usize).unwrap().chars().nth(y as usize).unwrap()
}
unsafe fn search(x:i32, y:i32, n:i32, m:i32, depth:i32, map:&Vec<&str>, mut v: &mut [[i32; 500]; 500]) {
    v[x as usize][y as usize] = depth;
    let c = get_char(x,y, &map);
    let ops = MY_CONST_MAP.get(&c).unwrap();
    for i in 0..4 {
        if (ops[i] == 1) {
            let nx:i32 = x + dx[i];
            let ny:i32 = y + dy[i];
            if nx>=0 && nx< n && ny>=0 && ny< m {
                if get_char(nx,ny, &map) != '.' && (v[nx as usize][ny as usize] == -1 || v[nx as usize][ny as usize] > depth+1) {
                    search(nx, ny, n, m, depth + 1, map, v);
                }
            }
        }
    }
}

static dx: &'static [i32] = &[0,1,0,-1];
static dy: &'static [i32] = &[-1,0,1,0];

lazy_static! {
    static ref MY_CONST_MAP: HashMap<&'static char, [i32;4]> = {
        let mut map: HashMap<&char, [i32;4]> = HashMap::new();
        map.insert(&'-', [1,0,1,0]);
        map.insert(&'|', [0,1,0,1]);
        map.insert(&'7', [1,1,0,0]);
        map.insert(&'J', [1,0,0,1]);
        map.insert(&'L', [0,0,1,1]);
        map.insert(&'F', [0,1,1,0]);
        map.insert(&'.', [0,0,0,0]);

        map.insert(&'S', [1,1,0,0]);
        map
    };
}

fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();
    let mut total:i32 = 0;
    let mut sX:i32 = 0;
    let mut sY:i32 = 0;
    let m:i32 = lines.len() as i32;
    let n: i32 = lines.first().unwrap().len() as i32;
    let mut visited : [[i32; 500]; 500] = [[-1; 500]; 500];
    for (i, s) in lines.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == 'S' {
                sX = i as i32;
                sY = j as i32;
            }
        }
    }
    unsafe {
        search(sX, sY, n,m, 0, &lines, &mut visited);
    }
    for i in 0..n{
        for j in 0..m {
            if (visited[i as usize][j as usize] > -1) {
                print!(".");
                print!("{}({})\t", visited[i as usize][j as usize], get_char(i, j, &lines));
                if total < visited[i as usize][j as usize] {
                    total = visited[i as usize][j as usize];
                }
            }
            else {
                print!("_");
            }
        }
        println!();
    }
    String::from(total.to_string())
}