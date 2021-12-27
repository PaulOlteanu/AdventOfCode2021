use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("inputs/day_07.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|age| age.parse::<i32>().unwrap())
        .collect();

    let num_positions = *input.iter().max().unwrap();
    let mut position_cost = vec![0; num_positions as usize + 1];

    for i in 0..=num_positions {
        let cost = input.iter().map(|start| (start - i).abs()).sum();
        position_cost[i as usize] = cost;
    }

    println!("{:#?}", position_cost.iter().min().unwrap());
}
