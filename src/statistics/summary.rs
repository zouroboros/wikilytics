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

pub fn find_nodes_of_max_out_degree(network: &HashMap<String, Vec<String>>) -> (Vec<&String>, usize) {
    let mut max_out_degree = 0;
    let mut nodes_of_max_out_degree = Vec::new();

    for (node, edges) in network {
        if edges.len() > max_out_degree {
            max_out_degree = edges.len();
            nodes_of_max_out_degree = vec![node];
        } else if edges.len() == max_out_degree {
            nodes_of_max_out_degree.push(node)
        }
    }

    (nodes_of_max_out_degree, max_out_degree)
}