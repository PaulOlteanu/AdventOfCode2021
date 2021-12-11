// use std::fs;
// use std::iter;

// use itertools::Itertools;

// //  0000
// // 1    2
// // 1    2
// //  3333
// // 4    5
// // 4    5
// //  6666

fn main() {}

// fn main() {
//     let input: Vec<(Vec<String>, Vec<String>)> = fs::read_to_string("inputs/day_8.txt")
//         .unwrap()
//         .trim()
//         .lines()
//         .map(|line| {
//             line.split('|')
//                 .map(|input| {
//                     input
//                         .trim()
//                         .split(' ')
//                         .map(|x| x.trim().to_owned())
//                         .collect_vec()
//                 })
//                 .collect_tuple()
//                 .unwrap()
//         })
//         .collect_vec();

//     let answer: i32 = input
//         .iter()
//         .map(|(input, output)| {
//             let mut connections: Vec<Vec<char>> = Vec::new();
//             // TODO: Add a "can be" and "can't be" vec
//             // Then do "can be" - "can't be" to find right connection for each
//             connections = (0..8)
//                 .map(|_| vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'])
//                 .collect_vec();
//             // input.iter().for_each(|digit| {
//             //     digit.chars().for_each(|c| {
//             //         if digit.len() == 2 {
//             //             connections[].retain(|x| x != c)
//             //         }
//             //     });
//             // });
//             println!("{:#?}", connections);
//             let num = output.iter().map(|digit| 1).collect_vec();
//             1
//         })
//         .sum();

//     println!("{:#?}", answer);
// }
