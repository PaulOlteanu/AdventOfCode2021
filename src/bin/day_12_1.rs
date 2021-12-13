use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    vertex: String,
    visited: HashSet<String>,
) -> u32 {
    if vertex == "end" {
        return 1;
    }

    let mut new_visited = visited.clone();
    if vertex == vertex.to_lowercase() {
        new_visited.insert(vertex.clone());
    }

    let connections = graph.get(&vertex).unwrap();
    let new_connections = connections
        .iter()
        .filter(|v| !visited.contains(*v))
        .collect_vec();
    if new_connections.is_empty() {
        return 0;
    }

    let mut result = 0;
    for connection in new_connections {
        result += count_paths(graph, connection.clone(), new_visited.clone());
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

    let result = count_paths(&graph, String::from("start"), HashSet::new());

    println!("{:#?}", graph);
    println!("{:#?}", result);
}
