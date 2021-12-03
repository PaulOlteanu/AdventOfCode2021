use std::fs;

const BIT_LENGTH: usize = 12;

// The index is 0-based, starting from the right (lsb)
fn get_most_common_bit(numbers: &[u32], index: usize) -> u32 {
    let count = numbers.len();
    let ones: u32 = numbers.iter().map(|&number| (number >> index) & 0b1).sum();

    if 2 * ones >= count as u32 {
        1
    } else {
        0
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day_3.txt").unwrap();

    let mut most_common: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line.trim(), 2).unwrap())
        .collect();

    let mut least_common = most_common.clone();

    let mut index = BIT_LENGTH;
    while most_common.len() > 1 {
        index -= 1;
        let mcb = get_most_common_bit(&most_common, index);
        most_common.retain(|&x| ((x >> index) & 1) == mcb);
    }

    index = BIT_LENGTH;
    while least_common.len() > 1 {
        index -= 1;
        let mcb = get_most_common_bit(&least_common, index);
        least_common.retain(|&x| ((x >> index) & 1) != mcb);
    }

    let oxygen_rating = most_common[0];
    let co2_rating = least_common[0];

    println!("Oxygen: {:?}", oxygen_rating);
    println!("CO2: {:?}", co2_rating);
    println!("Life Support: {:?}", oxygen_rating * co2_rating);
}
