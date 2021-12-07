use std::fs;

fn main() {
    let input: Vec<i64> = fs::read_to_string("inputs/day_7.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|age| age.parse::<i64>().unwrap())
        .collect();

    let num_positions = *input.iter().max().unwrap();
    let mut position_cost = vec![0; num_positions as usize + 1];

    for i in 0..=num_positions {
        let cost: i64 = input
            .iter()
            .map(|start| {
                let n = (start - i).abs();
                n * (n + 1) / 2
            })
            .sum();
        position_cost[i as usize] = cost;
    }

    println!("{:#?}", position_cost.iter().min().unwrap());
}
