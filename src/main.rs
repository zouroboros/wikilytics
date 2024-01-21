use std::fs::File;
use std::io::BufReader;

use bzip2::bufread::MultiBzDecoder;
use quick_xml::reader::Reader;

mod network_generator;
mod statistics;

use crate::network_generator::generate_network;
use crate::network_generator::wiki_pages::WikiPages;
use crate::statistics::gather_statistics;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let file = File::open("dewiki-20210801-pages-articles-multistream.xml.bz2")?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let pages = WikiPages::new(reader);

    let network = generate_network(pages);

    let statistics = gather_statistics(&network);

    println!("Network {:?}", statistics);

    Ok(())
}
