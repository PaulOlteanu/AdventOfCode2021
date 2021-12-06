use std::fs;

fn get_read_write<'a>(
    one: &'a mut [u64],
    two: &'a mut [u64],
    index: usize,
) -> (&'a mut [u64], &'a mut [u64]) {
    if index == 1 {
        (two, one)
    } else {
        (one, two)
    }
}

fn main() {
    let input: Vec<usize> = fs::read_to_string("inputs/day_6.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|age| age.parse::<usize>().unwrap())
        .collect();

    let mut one: [u64; 9] = [0u64; 9];
    let mut two: [u64; 9] = [0u64; 9];

    for age in input {
        one[age] += 1;
    }

    let mut write_index: usize = 2;
    for i in 0..256 {
        println!("On day: {}", i);

        let (read, write) = get_read_write(&mut one, &mut two, write_index);

        write[8] = read[0];
        write[7] = read[8];
        write[6] = read[7] + read[0];
        write[5] = read[6];
        write[4] = read[5];
        write[3] = read[4];
        write[2] = read[3];
        write[1] = read[2];
        write[0] = read[1];

        write_index = if write_index == 1 { 2 } else { 1 };
    }

    let read = if write_index == 1 { &mut two } else { &mut one };
    println!("Sum: {:#?}", read.iter().sum::<u64>());
}
