use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use bzip2::bufread::MultiBzDecoder;
use quick_xml::reader::Reader;

mod network_generator;

use crate::network_generator::generate_network;
use crate::network_generator::wiki_pages::WikiPages;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let file = File::open("simplewiki-20230820-pages-articles-multistream.xml.bz2")?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let pages = WikiPages::new(reader);

    let network = generate_network(pages);

    let adjacency_file = File::create("adjacency_file")?;
    let mut adjacency_writer = BufWriter::new(adjacency_file);

    for (page, links) in network {
        adjacency_writer.write_all(page.as_bytes())?;
        for link in links {
            adjacency_writer.write_all(",".as_bytes())?;
            adjacency_writer.write_all(link.as_bytes())?;
        }
        adjacency_writer.write_all("\n".as_bytes())?;
    }

    Ok(())
}
