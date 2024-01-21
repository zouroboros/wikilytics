use std::collections::HashMap;
use self::summary::{number_of_edges, number_of_nodes, find_nodes_of_max_out_degree};

pub mod summary;

#[derive(Debug)]
pub struct NetworkStatistics<'a> {
    number_of_nodes: usize,
    number_of_edges: usize,
    nodes_of_max_out_degree: Vec<&'a String>,
    max_out_degree: usize
}

pub fn gather_statistics(network: &HashMap<String, Vec<String>>) -> NetworkStatistics {
    let (nodes_of_max_out_degree, max_out_degree) = find_nodes_of_max_out_degree(network);

    NetworkStatistics {
        number_of_nodes: number_of_nodes(network),
        number_of_edges: number_of_edges(network),
        nodes_of_max_out_degree,
        max_out_degree
    }
}