use std::fs::File;
use std::io::BufReader;

use bzip2::bufread::MultiBzDecoder;
use quick_xml::reader::Reader;

mod network_generator;
mod statistics;

use crate::network_generator::generate_network;
use crate::network_generator::wiki_pages::WikiPages;
use crate::statistics::summary;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let file = File::open("enwiki-20231220-pages-articles-multistream.xml.bz2")?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let pages = WikiPages::new(reader);

    let network = generate_network(pages);

   let number_of_nodes = summary::number_of_nodes(&network);
   let number_of_edges = summary::number_of_edges(&network);

    println!("Network containes {} nodes and {} edges", number_of_nodes, number_of_edges);

    Ok(())
}
