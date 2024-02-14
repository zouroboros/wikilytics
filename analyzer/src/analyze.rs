use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, BufWriter, Result}, path::PathBuf};

use crate::statistics::gather_statistics;

pub fn analyze(network_file_path: PathBuf, statistics_file_path: PathBuf) -> Result<()> {
    let network = load_network(network_file_path)?;
    let statistics = gather_statistics(&network);
    let statistics_file = File::create(statistics_file_path)?;
    let statistics_writer = BufWriter::new(statistics_file);

    serde_json::to_writer(statistics_writer, &statistics)?;
    Ok(())
}

fn load_network(network_file_path: PathBuf) -> Result<HashMap<String, Vec<String>>> {
    let network_file = File::open(network_file_path)?;
    let network_file_reader = BufReader::new(network_file);

    let mut network = HashMap::new();

    for line_result in network_file_reader.lines() {
        let line = line_result?;

        let mut entries = line.split(",");
        if let Some(node) = entries.next() {
            network.insert(node.to_owned(), Vec::from_iter(entries.map(|s| s.to_owned())));
        }
        
    }

    return Ok(network);
}