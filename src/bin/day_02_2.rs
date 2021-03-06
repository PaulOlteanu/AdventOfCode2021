use std::fs;

struct PositionData {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

fn main() {
    let input = fs::read_to_string("inputs/day_02.txt").unwrap();

    let mut position = PositionData {
        horizontal: 0,
        vertical: 0,
        aim: 0,
    };

    input.lines().for_each(|s| {
        let mut spl = s.split(' ');
        let (direction, amount) = (
            spl.next().unwrap(),
            spl.next().unwrap().parse::<i32>().unwrap(),
        );
        match direction {
            "up" => position.aim += amount,
            "down" => position.aim -= amount,
            "forward" => {
                position.horizontal += amount;
                position.vertical += position.aim * amount;
            }
            _ => println!("Unknown direction"),
        }
    });

    println!("{:?}", position.horizontal * position.vertical);
}
