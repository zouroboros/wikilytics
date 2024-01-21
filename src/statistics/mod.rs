use std::collections::HashMap;
use self::summary::{number_of_edges, number_of_nodes};

pub mod summary;

#[derive(Debug)]
pub struct NetworkStatistics {
    number_of_nodes: usize,
    number_of_edges: usize
}

pub fn gather_statistics(network: &HashMap<String, Vec<String>>) -> NetworkStatistics {
    NetworkStatistics {
        number_of_nodes: number_of_nodes(network),
        number_of_edges: number_of_edges(network)
    }
}