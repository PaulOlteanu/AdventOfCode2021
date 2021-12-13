use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    vertex: String,
    visited: HashMap<String, u32>,
    path: Vec<String>,
) -> u32 {
    let mut path = path;
    path.push(vertex.clone());

    if vertex == "end" {
        return 1;
    }

    let mut new_visited = visited;
    *new_visited.entry(vertex.clone()).or_insert(0) += 1;
    let any_small_doubled = new_visited
        .clone()
        .iter()
        .any(|(k, v)| k.to_ascii_uppercase() != *k && *v == 2);

    let connections = graph.get(&vertex).unwrap();
    let new_connections = connections
        .iter()
        .filter(|&v| {
            if v == "start" {
                return false;
            }

            let times_visited = *new_visited.entry(v.clone()).or_insert(0);

            (v.to_uppercase() == *v)
                || (times_visited == 0)
                || (!any_small_doubled && times_visited == 1)
        })
        .collect_vec();

    if new_connections.is_empty() {
        return 0;
    }

    let mut result = 0;
    for connection in new_connections {
        result += count_paths(graph, connection.clone(), new_visited.clone(), path.clone());
    }

    result
}

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    fs::read_to_string("inputs/day_12.txt")
        .unwrap()
        .trim()
        .lines()
        .for_each(|line| {
            let (a, b) = line.trim().split('-').collect_tuple().unwrap();
            let entry = graph.entry(a.to_owned()).or_insert_with(Vec::new);
            entry.push(b.to_owned());

            let entry = graph.entry(b.to_owned()).or_insert_with(Vec::new);
            entry.push(a.to_owned());
        });

    let result = count_paths(&graph, String::from("start"), HashMap::new(), Vec::new());

    println!("{:#?}", result);
}
