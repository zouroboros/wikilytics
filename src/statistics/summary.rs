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

pub fn calculate_out_degrees(network: &HashMap<String, Vec<String>>) -> HashMap<&String, usize> {
    let mut out_degrees = HashMap::with_capacity(network.capacity());

    for (node, edges) in network {
        out_degrees.insert(node, edges.len());
    }

    out_degrees
}

pub fn calculate_in_degrees(network: &HashMap<String, Vec<String>>) -> HashMap<&String, usize> {
    let mut in_degrees = HashMap::with_capacity(network.capacity());

    for (_, edges) in network {
        for linked_node in edges {
            in_degrees.insert(linked_node, in_degrees.get(linked_node)
                .map_or(1, |count| count + 1));
        }
    }

    in_degrees
}

pub fn find_max_degree(degrees: HashMap<&String, usize>) -> (Vec<&String>, usize) {
    let mut max_degree = 0;
    let mut nodes_of_max_degree = Vec::new();

    for (node, degree) in degrees {
        if degree > max_degree {
            max_degree = degree;
            nodes_of_max_degree = vec![node];
        } else if degree == max_degree {
            nodes_of_max_degree.push(node)
        }
    }

    (nodes_of_max_degree, max_degree)
}