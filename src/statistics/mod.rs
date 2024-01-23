use std::collections::HashMap;
use serde::Serialize;

use self::summary::{number_of_edges, number_of_nodes, calculate_out_degrees, calculate_in_degrees, find_max_degree, degree_histogram};

pub mod summary;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkStatistics {
    number_of_nodes: usize,
    number_of_edges: usize,
    nodes_of_max_out_degree: Vec<String>,
    max_out_degree: usize,
    nodes_of_max_in_degree: Vec<String>,
    max_in_degree: usize,
    out_degree_distribution: Vec<usize>,
    in_degree_distribution: Vec<usize>
}

pub fn gather_statistics(network: &HashMap<String, Vec<String>>) -> NetworkStatistics {
    let out_degrees = calculate_out_degrees(network);
    let in_degrees = calculate_in_degrees(network);
    let (nodes_of_max_out_degree, max_out_degree) = find_max_degree(&out_degrees);
    let (nodes_of_max_in_degree, max_in_degree) = find_max_degree(&in_degrees);
    let out_degree_distribution = degree_histogram(&out_degrees, max_out_degree);
    let in_degree_distribution = degree_histogram(&in_degrees, max_in_degree);

    NetworkStatistics {
        number_of_nodes: number_of_nodes(network),
        number_of_edges: number_of_edges(network),
        nodes_of_max_out_degree,
        max_out_degree,
        nodes_of_max_in_degree,
        max_in_degree,
        out_degree_distribution,
        in_degree_distribution
    }
}