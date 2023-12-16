use std::collections::{HashMap};
use std::hash::Hash;
use std::usize;
use std::cmp;

fn main() {
    let input = include_str!("input1.txt");
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

    let mut total:i32 = 0;

    let mut rows: Vec<i32> = Vec::new();
    let mut cols: Vec<i32> = Vec::new();
    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    let mut cnt = 0;
    let n = map.len();
    let m = map[0].len();
    for i in 0..n {
        let mut found = false;
        for j in 0..m {
            if (map[i][j] == '#') {
                galaxies.push((i as i32,j as i32));
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

    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len(){
            let mut dist = i32::abs(galaxies[i].0 - galaxies[j].0) + i32::abs(galaxies[i].1 - galaxies[j].1);
            let maxX = cmp::max(galaxies[i].0, galaxies[j].0);
            let minX = cmp::min(galaxies[i].0, galaxies[j].0);
            let maxY = cmp::max(galaxies[i].1, galaxies[j].1);
            let minY = cmp::min(galaxies[i].1, galaxies[j].1);
            dist+=rows[maxX as usize] - rows[minX as usize];
            dist+=cols[maxY as usize] - cols[minY as usize];
            total+=dist;
        }
    }
    String::from(total.to_string())
}