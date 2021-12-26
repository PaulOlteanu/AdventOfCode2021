use std::fs;
use std::fs::File;
use std::io::Write;

use itertools::Itertools;

fn main() {
    let mut f = File::create("./outputs/day_24.txt").unwrap();
    fs::read_to_string("inputs/day_24.txt")
        .unwrap()
        .trim()
        .lines()
        .for_each(|line| {
            let mut spl = line.split(' ');
            let op = spl.next().unwrap();
            match op {
                "inp" => {
                    let a = spl.next().unwrap();
                    let data = format!("\n{} = input()\n", a);
                    f.write_all(data.as_bytes()).unwrap();
                }
                "add" => {
                    let (a, b) = spl.collect_tuple().unwrap();
                    let data = format!("{} = {} + {}\n", a, a, b);
                    f.write_all(data.as_bytes()).unwrap();
                }
                "mul" => {
                    let (a, b) = spl.collect_tuple().unwrap();
                    let data = format!("{} = {} * {}\n", a, a, b);
                    f.write_all(data.as_bytes()).unwrap();
                }
                "div" => {
                    let (a, b) = spl.collect_tuple().unwrap();
                    let data = format!("{} = {} / {}\n", a, a, b);
                    f.write_all(data.as_bytes()).unwrap();
                }
                "mod" => {
                    let (a, b) = spl.collect_tuple().unwrap();
                    let data = format!("{} = {} % {}\n", a, a, b);
                    f.write_all(data.as_bytes()).unwrap();
                }
                "eql" => {
                    let (a, b) = spl.collect_tuple().unwrap();
                    let data = format!("{} = 1 if {} == {} else 0\n", a, a, b);
                    f.write_all(data.as_bytes()).unwrap();
                }
                _ => unreachable!(),
            }
        });
}

/*
OP: [1,   1,  1, 26, 26, 1, 26, 26, 1, 26, 1, 1, 26, 26]

//         0   1   2   3   4   5   6    7   8    9  10  11  12  13
// PUSH: [15, 12, 15, 12, 15,  2, 11,  15, 10,   2,  0,  0, 15, 15];
// POP:  [12, 14, 11, -9, -7, 11, -1, -16, 11, -15, 10, 12, -4,  0];

PUSH A + 15
PUSH B + 12
PUSH C + 15
POP C + 15, C + 15 - 9 = D
POP B + 12, B + 12 - 7 = E
PUSH F + 2
POP F + 2, F + 2 - 1 = G
POP A + 15, A + 15 - 16 = H
PUSH I + 10
POP I + 10, I + 10 - 15 = J
PUSH K
PUSH L
POP L, L - 4 = M
POP K, K = N
*/
