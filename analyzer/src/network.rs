use std::{collections::HashMap, fs::File, io::{BufReader, BufWriter, Write}, path::PathBuf};
use std::io::Result;

use bzip2::bufread::MultiBzDecoder;
use quick_xml::Reader;

use crate::network_generator::{generate_network, wiki_xml_dump::WikiXmlDump};

pub fn network(path: PathBuf, network_file_path: PathBuf) -> Result<()> {
    let file = File::open(path)?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let xml_dump = WikiXmlDump::new(reader);

    //let base = xml_dump.read_base().unwrap();

    let network = generate_network(xml_dump);

    save_network(network, network_file_path)?;

    /*let statistics = gather_statistics(&network, base);

    let statistics_file = File::create("../viewer/public/statistics.json")?;
    let statistics_writer = BufWriter::new(statistics_file);

    serde_json::to_writer(statistics_writer, &statistics)?;*/

    Ok(())
}

fn save_network(network_to_save: HashMap<String, Vec<String>>, save_file_path: PathBuf) -> Result<()> {
    let save_file = File::create(save_file_path)?;
    let mut file_writer = BufWriter::new(save_file);

    let separator = "; ".as_bytes();
    let newline = "\n".as_bytes();

    for (node, connected_nodes) in network_to_save {
        file_writer.write(node.as_bytes())?;

        for connected_node in connected_nodes {
            file_writer.write(separator)?;
            file_writer.write(connected_node.as_bytes())?;
        }

        file_writer.write(newline)?;
    }

    Ok(())
}