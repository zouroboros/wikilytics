use std::collections::HashMap;
use self::summary::{number_of_edges, number_of_nodes, calculate_out_degrees, calculate_in_degrees, find_max_degree};

pub mod summary;

#[derive(Debug)]
pub struct NetworkStatistics<'a> {
    number_of_nodes: usize,
    number_of_edges: usize,
    nodes_of_max_out_degree: Vec<&'a String>,
    max_out_degree: usize,
    nodes_of_max_in_degree: Vec<&'a String>,
    max_in_degree: usize
}

pub fn gather_statistics(network: &HashMap<String, Vec<String>>) -> NetworkStatistics {
    let out_degrees = calculate_out_degrees(network);
    let in_degrees = calculate_in_degrees(network);
    let (nodes_of_max_out_degree, max_out_degree) = find_max_degree(out_degrees);
    let (nodes_of_max_in_degree, max_in_degree) = find_max_degree(in_degrees);

    NetworkStatistics {
        number_of_nodes: number_of_nodes(network),
        number_of_edges: number_of_edges(network),
        nodes_of_max_out_degree,
        max_out_degree,
        nodes_of_max_in_degree,
        max_in_degree
    }
}