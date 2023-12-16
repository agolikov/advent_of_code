use std::collections::{HashMap};
use std::hash::Hash;
use std::usize;
use std::cmp;

fn main() {
    let input = include_str!("input2.txt");
    let output = sol(input);
    println!("{}", output);
}

fn get_char(x:i32, y:i32, map:&Vec<&str>) -> char{
    map.get(x as usize).unwrap().chars().nth(y as usize).unwrap()
}

fn sol(input: &str) -> String {
    let lines: Vec<_> = input.split('\n').collect();

    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut total:i64 = 0;

    let mut rows: Vec<i64> = Vec::new();
    let mut cols: Vec<i64> = Vec::new();
    let mut galaxies: Vec<(i64, i64)> = Vec::new();

    let mut cnt = 0;
    let n = map.len();
    let m = map[0].len();
    for i in 0..n {
        let mut found = false;
        for j in 0..m {
            if (map[i][j] == '#') {
                galaxies.push((i as i64,j as i64));
                found = true;
            }
        }
        if (!found) {
            cnt += 1;
        }
        rows.push(cnt);
    }
    cnt = 0;
    for i in 0..m {
        let mut found = false;
        for j in 0..n {
            if (map[j][i] == '#') {
                found = true;
                break;
            }
        }
        if (!found) {
            cnt += 1;
        }
        cols.push(cnt);
    }

    let scale = 1000000 - 1;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let x1:i64 = galaxies[i].0 as i64 + rows[galaxies[i].0 as usize]*scale;
            let y1:i64 = galaxies[i].1 as i64 + cols[galaxies[i].1 as usize]*scale;
            let x2:i64 = galaxies[j].0 as i64 + rows[galaxies[j].0 as usize]*scale;
            let y2:i64 = galaxies[j].1 as i64 + cols[galaxies[j].1 as usize]*scale;
            let mut dist = i64::abs(x1-x2 ) + i64::abs(y1-y2);
            total+=dist;
        }
    }
    String::from(total.to_string())
}