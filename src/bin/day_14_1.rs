use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn main() {
    let input = fs::read_to_string("inputs/day_14.txt").unwrap();
    let mut input = input.trim().lines();

    let polymer = input.next().unwrap().to_owned();
    input.next();

    let mut rules: HashMap<String, char> = HashMap::new();

    input.for_each(|line| {
        let (a, b) = line.split(" -> ").collect_tuple().unwrap();
        rules.insert(a.to_owned(), b.chars().next().unwrap());
    });

    let mut pair_counts: HashMap<String, u32> = HashMap::new();
    let mut char_counts: HashMap<char, u32> = HashMap::new();
    let mut next: HashMap<String, u32> = HashMap::new();

    polymer.chars().tuple_windows().for_each(|(a, b)| {
        let key = format!("{}{}", a, b);
        *pair_counts.entry(key).or_insert(0) += 1;
    });

    polymer.chars().for_each(|c| {
        *char_counts.entry(c).or_insert(0) += 1;
    });

    for i in 0..10 {
        println!("Processing step {}", i + 1);
        pair_counts.iter().for_each(|(pair, count)| {
            let insert = rules.get(pair).unwrap();
            *char_counts.entry(*insert).or_insert(0) += count;

            let (char1, char2) = pair.chars().collect_tuple().unwrap();
            let pair1 = format!("{}{}", char1, insert);
            let pair2 = format!("{}{}", insert, char2);
            *next.entry(pair1).or_insert(0) += count;
            *next.entry(pair2).or_insert(0) += count;
        });

        pair_counts = next;
        next = HashMap::new();
    }

    match char_counts.iter().minmax_by_key(|(_, v)| *v) {
        MinMax((_, min), (_, max)) => {
            println!("{}", max - min);
        }
        _ => unreachable!(),
    }
}
