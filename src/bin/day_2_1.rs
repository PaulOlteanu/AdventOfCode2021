use std::fs;

struct PositionData {
    horizontal: i32,
    vertical: i32,
}

fn main() {
    let input = fs::read_to_string("inputs/day_2_1.txt").unwrap();

    let mut position = PositionData {
        horizontal: 0,
        vertical: 0,
    };

    input.lines().for_each(|s| {
        let mut spl = s.split(' ');
        let (direction, amount) = (
            spl.next().unwrap(),
            spl.next().unwrap().parse::<i32>().unwrap(),
        );
        match direction {
            "up" => position.vertical += amount,
            "down" => position.vertical -= amount,
            "forward" => position.horizontal += amount,
            _ => println!("Unknown direction"),
        }
    });

    println!("{:?}", position.horizontal * position.vertical);
}
