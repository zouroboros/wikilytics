use std::fs::File;
use std::io::{BufReader, BufWriter};

use bzip2::bufread::MultiBzDecoder;
use quick_xml::reader::Reader;

mod network_generator;
mod statistics;

use crate::network_generator::generate_network;
use crate::network_generator::wiki_xml_dump::WikiXmlDump;
use crate::statistics::gather_statistics;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let file = File::open("simplewiki-20230820-pages-articles-multistream.xml.bz2")?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let mut xml_dump = WikiXmlDump::new(reader);

    let base = xml_dump.read_base().unwrap();

    let network = generate_network(xml_dump);

    let statistics = gather_statistics(&network, base);

    let statistics_file = File::create("../viewer/public/statistics.json")?;
    let statistics_writer = BufWriter::new(statistics_file);

    serde_json::to_writer(statistics_writer, &statistics)?;

    Ok(())
}
