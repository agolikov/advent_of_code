use std::{cmp, fmt};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ptr::hash;

fn main() {
    let input = include_str!("input1.txt");
    let output = sol(input);
    println!("{}", output);
}
struct Hand
{
    text:String,
    wins:u64
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl ToString for HandType {
    fn to_string(&self) -> String {
        match self {
            HandType::FiveOfAKind => String::from("FiveOfAKind"),
            HandType::FourOfAKind => String::from("FourOfAKind"),
            HandType::FullHouse => String::from("FullHouse"),
            HandType::ThreeOfAKind => String::from("ThreeOfAKind"),
            HandType::TwoPair => String::from("TwoPair"),
            HandType::OnePair => String::from("OnePair"),
            HandType::HighCard => String::from("HighCard")
        }
    }
}



fn hand_type(hand: &str) -> HandType {
    let mut counts = [0; 13];

    for c in hand.chars() {
        match c {
            'A' => counts[12] += 1,
            'K' => counts[11] += 1,
            'Q' => counts[10] += 1,
            'J' => counts[9] += 1,
            'T' => counts[8] += 1,
            '9' => counts[7] += 1,
            '8' => counts[6] += 1,
            '7' => counts[5] += 1,
            '6' => counts[4] += 1,
            '5' => counts[3] += 1,
            '4' => counts[2] += 1,
            '3' => counts[1] += 1,
            '2' => counts[0] += 1,
            _ => panic!("Invalid card label"),
        }
    }
    let max_count = *counts.iter().max().unwrap();
    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            for i in 0..counts.len() {
                if counts[i] == 2 {
                    return HandType::FullHouse;
                }
            }
            HandType::ThreeOfAKind
        },
        2 => {
            let mut c: u32 = 0;
            for i in 0..counts.len() {
                if counts[i] == 2 {
                    c += 1;
                }
                if c == 2 {
                    return HandType::TwoPair;
                }
            }
            HandType::OnePair
        },
        _ => HandType::HighCard,
    }
}

fn compare_hands(hand1: &str, hand2: &str, ws:&HashMap<char, i32>) -> Ordering {
    let type1 = hand_type(hand1);
    let type2 = hand_type(hand2);

    if type1 != type2 {
        return type2.cmp(&type1);
    }
    for id in 0..hand1.len() {
        let char_vec1: Vec<char> = hand1.chars().collect();
        let char_vec2: Vec<char> = hand2.chars().collect();
        let v1:i32 = ws[&char_vec1[id]];
        let v2:i32 = ws[&char_vec2[id]];
        let order = v1.cmp(&v2);
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn sol(input: &str) -> String {
    let weights = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('J', 9),
        ('T', 8),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);

    let lines: Vec<_> = input.split('\n').collect();
    let mut hands: Vec<Hand> = Vec::new();
    for l in lines {
        let text = &l[0..5];
        let wins = &l[6..].parse::<u64>().unwrap();
        hands.push(Hand{text: text.parse().unwrap(), wins:*wins});
    }
    hands.sort_by(|a, b| compare_hands(&a.text, &b.text, &weights));

    let mut total: u64 = 0;
    for r in 0..hands.len() {
        total += (r as u64 + 1) * hands[r].wins;
        //println!("{} {}", hands[r].text, hand_type(&hands[r].text).to_string());
    }
    String::from(total.to_string())
}