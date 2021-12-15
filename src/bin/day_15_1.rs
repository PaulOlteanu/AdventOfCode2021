use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn dijkstras(graph: &[Vec<u32>], src: (usize, usize), dest: (usize, usize)) -> u32 {
    let max_y = graph.len();
    let max_x = graph[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut costs = (0..max_y).map(|_| vec![u32::MAX; max_x]).collect_vec();

    let (start_x, start_y) = src;
    let (end_x, end_y) = dest;
    costs[start_y][start_x] = 0;

    let mut current = src;

    let mut queue = HashMap::new();
    queue.insert(src, 0);

    loop {
        visited.insert(current);

        let mut connections = Vec::new();
        let (current_x, current_y) = current;
        if current_x != 0 {
            connections.push((current_x - 1, current_y))
        }
        if current_y != 0 {
            connections.push((current_x, current_y - 1))
        }
        if current_x != max_x - 1 {
            connections.push((current_x + 1, current_y))
        }
        if current_y != max_y - 1 {
            connections.push((current_x, current_y + 1))
        }

        for (cx, cy) in &connections {
            if costs[current_y][current_x] + graph[*cy][*cx] < costs[*cy][*cx] {
                costs[*cy][*cx] = costs[current_y][current_x] + graph[*cy][*cx];
            }
        }

        connections.retain(|x| !visited.contains(x));

        for (cx, cy) in &connections {
            *queue.entry((*cx, *cy)).or_insert(0) = costs[*cy][*cx];
        }

        let temp = queue.clone();
        if let Some((n, _)) = temp.iter().min_by_key(|&(_, c)| *c) {
            queue.remove(n);
            current = *n;
        } else {
            break;
        }
    }

    costs[end_y][end_x]
}

fn main() {
    let input = fs::read_to_string("inputs/day_15.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let y_max = input.len();
    let x_max = input[0].len();

    let result = dijkstras(&input, (0, 0), (x_max - 1, y_max - 1));

    println!("{}", result);
}
