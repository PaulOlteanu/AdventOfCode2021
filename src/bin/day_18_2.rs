use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
enum SNumber {
    Literal(u32),
    Pair(Pair),
}

#[derive(Debug)]
struct Pair {
    left: Box<SNumber>,
    right: Box<SNumber>,
}

impl Pair {
    fn magnitude(&self) -> u32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

impl SNumber {
    fn parse(reader: &mut VecDeque<char>) -> Self {
        // Pop open bracket
        reader.pop_front().unwrap();

        let c = *reader.get(0).unwrap();
        let left = if c == '[' {
            SNumber::parse(reader)
        } else {
            reader.pop_front().unwrap();
            let v = c.to_digit(10).unwrap();
            SNumber::Literal(v)
        };

        // Pop comma
        reader.pop_front().unwrap();

        let c = *reader.get(0).unwrap();
        let right = if c == '[' {
            SNumber::parse(reader)
        } else {
            reader.pop_front().unwrap();
            let v = c.to_digit(10).unwrap();
            SNumber::Literal(v)
        };

        // Pop closing bracket
        reader.pop_front().unwrap();

        SNumber::Pair(Pair {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Literal(v) => *v,
            Self::Pair(pair) => pair.magnitude(),
        }
    }
}

fn add(list1: &[(u32, u32)], list2: &[(u32, u32)]) -> Vec<(u32, u32)> {
    list1
        .iter()
        .chain(list2.iter())
        .map(|(v, d)| (*v, d + 1))
        .collect()
}

fn explodable(list: &[(u32, u32)]) -> bool {
    list.iter().any(|(_, depth)| *depth >= 5)
}

fn explode(list: &mut Vec<(u32, u32)>) {
    for i in 0..list.len() {
        let (val, depth) = list[i];
        if depth >= 5 && i != list.len() - 1 {
            let (val2, depth2) = list[i + 1];
            if depth == depth2 {
                if i != 0 {
                    let (prev_val, prev_depth) = list[i - 1];
                    list[i - 1] = (prev_val + val, prev_depth)
                }

                if i != list.len() - 2 {
                    let (next_val, next_depth) = list[i + 2];
                    list[i + 2] = (next_val + val2, next_depth)
                }

                list.remove(i);
                list.remove(i);
                list.insert(i, (0, depth - 1));
                return;
            }
        }
    }
}

fn splitable(list: &[(u32, u32)]) -> bool {
    list.iter().any(|(val, _)| *val >= 10)
}

fn split(list: &mut Vec<(u32, u32)>) {
    for i in 0..list.len() {
        let (val, depth) = list[i];
        if val >= 10 {
            list.remove(i);
            list.insert(i, (val / 2, depth + 1));
            list.insert(i + 1, ((val as f64 / 2.0).ceil() as u32, depth + 1));
            return;
        }
    }
}

#[allow(clippy::same_item_push)]
fn format(list: &[(u32, u32)]) -> String {
    let mut result = String::from("");

    let mut depth = 0;
    let mut num_written = vec![0];

    for &(v, d) in list {
        if d > depth {
            let range = depth..d;
            for _ in range {
                result.push('[');
                depth += 1;
                num_written.push(0);
            }
        }

        result.push(((48 + v) as u8) as char);
        *num_written.last_mut().unwrap() += 1;

        while *num_written.last().unwrap() == 2 {
            num_written.pop();
            result.push(']');
            depth -= 1;
            *num_written.last_mut().unwrap() += 1;
        }

        result.push(',');
    }

    result
}

fn magnitude(list: &[(u32, u32)]) -> u32 {
    let s = format(list);
    let mut s = s.chars().collect::<VecDeque<char>>();
    let snumber = SNumber::parse(&mut s);
    snumber.magnitude()
}

fn reduce(list: &mut Vec<(u32, u32)>) {
    loop {
        while explodable(list) {
            explode(list);
        }

        if !splitable(list) {
            break;
        }

        split(list);
    }
}

fn main() {
    let input: Vec<Vec<(u32, u32)>> = fs::read_to_string("inputs/day_18.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut result = Vec::new();
            let mut depth = 0;
            line.chars().for_each(|c| {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                } else if c != ',' {
                    let val = c.to_digit(10).unwrap();
                    result.push((val, depth));
                }
            });

            result
        })
        .collect();

    let mut max_magnitude = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j {
                let mut sum = add(&input[i], &input[j]);
                reduce(&mut sum);
                let mag = magnitude(&sum);
                if mag > max_magnitude {
                    max_magnitude = mag;
                }
                let mut sum = add(&input[j], &input[i]);
                reduce(&mut sum);
                let mag = magnitude(&sum);
                if mag > max_magnitude {
                    max_magnitude = mag;
                }
            }
        }
    }

    println!("{:?}", max_magnitude);
}
