use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use bzip2::bufread::{MultiBzDecoder};
use quick_xml::reader::Reader;

use crate::wiki_pages::WikiPages;
use crate::wiki_text::linked_articles;

mod wiki_pages;
mod wiki_text;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let file = File::open("enwiki-20231220-pages-articles-multistream.xml.bz2")?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let pages = WikiPages::new(reader);

    let adjacency_file = File::create("adjacency_file")?;
    let mut adjacency_writer = BufWriter::new(adjacency_file);

    for page in pages {
        adjacency_writer.write_all(page.title.as_bytes())?;
        for linked_page in linked_articles(&page) {
            adjacency_writer.write_all(",".as_bytes())?;
            adjacency_writer.write_all(linked_page.as_bytes())?;
        }
        adjacency_writer.write_all("\n".as_bytes())?;
    }

    Ok(())
}
