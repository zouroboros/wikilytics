use std::collections::HashMap;

pub fn number_of_nodes(network: &HashMap<String, Vec<String>>) -> usize {
    network.len()
}

pub fn number_of_edges(network: &HashMap<String, Vec<String>>) -> usize {
    let mut number_of_edges = 0;

    for (_, edges) in network {
        number_of_edges = number_of_edges + edges.len();
    }

    number_of_edges
}