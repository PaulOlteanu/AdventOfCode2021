use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_03.txt").unwrap();

    let mut sum = [0; 12];
    let mut count = 0;

    input.lines().for_each(|line| {
        count += 1;

        let mut value = u32::from_str_radix(line.trim(), 2).unwrap();

        for i in 0..12 {
            sum[11 - i] += value & 0b1;
            value >>= 1;
        }
    });

    let threshold = count / 2;

    let mut gamma: u32 = 0;
    for bit in sum {
        let bit = if bit > threshold { 1 } else { 0 };
        gamma += bit;
        gamma <<= 1;
    }
    gamma >>= 1;

    let mask = (1 << 12) - 1;
    let epsilon = (!gamma) & mask;

    println!("{:?}", epsilon * gamma);
}
